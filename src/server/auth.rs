use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::{FromRef, FromRequestParts, Query, State};
use axum::http::header::SET_COOKIE;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, RequestPartsExt, Router};
use axum_extra::TypedHeader;
use axum_extra::headers::Cookie;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use gluer::metadata;
use hyper::{HeaderMap, StatusCode};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, AuthorizationRequest, ClientId, ClientSecret, CodeTokenRequest,
    CsrfToken, EndpointNotSet, EndpointSet, RedirectUrl, RevocationUrl, Scope, TokenResponse,
    TokenUrl,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

use crate::error::{Error, Result};

static COOKIE_NAME: &str = "SESSION";
const SESSION_CHECK_SEC: u64 = 8 * 60 * 60; // 8h
const SESSION_EXPIRE_SEC: u64 = 2 * 24 * 60 * 60; // 2d

const LOGIN_COUNT: usize = 10;
const LOGIN_EXPIRE_SEC: u64 = 5 * 60;

/// Configuration for OAuth
#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    /// The application id
    pub client_id: String,
    /// The application secret
    pub client_secret: String,
    /// Login page from the OAuth server
    pub auth_url: String,
    /// Endpoint for converting the login code to a token
    pub token_url: String,
    /// Endpoint for revoking an access token
    pub revoke_url: Option<String>,

    /// Endpoint for user data (requires a token)
    pub profile_url: String,
    /// Required scope for the identity route.
    /// - Discord: "identify"
    /// - Iserv: "profile"
    pub profile_scope: String,
    /// Key in the json dictionary returned by the identity route.
    /// This can also be a dot separated list of keys into nested dictionaries.
    /// - Discord: "id"
    /// - Iserv: "preferred_username"
    pub profile_key: String,
}

/// The routes required for login and logout
pub fn routes(auth: Auth) -> Router {
    match auth {
        Auth::None => Router::new(),
        Auth::OAuth(auth) => Router::new()
            .route("/login", get(login_redirect))
            .route("/authorized", get(login_authorized))
            .route("/logout", get(logout))
            .fallback(|| async { (StatusCode::NOT_FOUND, Json(Error::NothingFound)) })
            .with_state(auth),
    }
}

/// Background task to clean up expired sessions
pub async fn background(auth: Auth) {
    if let Auth::OAuth(auth) = auth {
        let mut timer = tokio::time::interval(Duration::from_secs(SESSION_CHECK_SEC));
        loop {
            timer.tick().await;

            let auth = auth.clone();
            tokio::task::spawn(async move {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                auth.sessions
                    .write()
                    .unwrap()
                    .retain(|_, l| l.expires > now);
                auth.logins.lock().unwrap().retain(|t| t.1 > now);
            });
        }
    }
}

/// The user data for a successful login.
#[metadata]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Login {
    /// A unique user id
    /// - Discord: https://discord.com/developers/docs/resources/user#user-object-user-structure
    /// - Iserv: https://doku.iserv.de/manage/system/sso
    pub id: String,
    /// Custom data storing how long the session is valid
    pub expires: u64,
}

/// The authentication method used by the server
#[derive(Debug, Clone)]
pub enum Auth {
    None,
    OAuth(Arc<OAuthState>),
}

impl Auth {
    /// Initialize the authentication.
    ///
    /// This requires the public domain of the webserver used for redirections
    pub fn new(domain: &str, config: Option<AuthConfig>) -> Self {
        if let Some(AuthConfig {
            client_id,
            client_secret,
            auth_url,
            token_url,
            revoke_url,
            profile_url,
            profile_scope,
            profile_key,
        }) = config
        {
            let redirect = format!("https://{domain}/auth/authorized");
            let client = BasicClient::new(ClientId::new(client_id))
                .set_client_secret(ClientSecret::new(client_secret))
                .set_auth_uri(AuthUrl::new(auth_url).unwrap())
                .set_token_uri(TokenUrl::new(token_url).unwrap())
                .set_redirect_uri(RedirectUrl::new(redirect).unwrap());

            let client = if let Some(revoke_url) = revoke_url {
                Client::Revokable(
                    client.set_revocation_url(RevocationUrl::new(revoke_url).unwrap()),
                )
            } else {
                Client::NonRevokable(client)
            };

            Self::OAuth(Arc::new(OAuthState {
                client,
                sessions: Default::default(),
                logins: Default::default(),
                profile_url,
                profile_scope,
                profile_key,
            }))
        } else {
            error!("SECURITY: Missing OAuth configuration!");
            Auth::None
        }
    }
}

/// Client wrapper that can be revokable or not
enum Client {
    Revokable(BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>),
    NonRevokable(
        BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    ),
}
impl Client {
    fn authorize_url(&'_ self, csrf: impl FnOnce() -> CsrfToken) -> AuthorizationRequest<'_> {
        match self {
            Client::Revokable(client) => client.authorize_url(csrf),
            Client::NonRevokable(client) => client.authorize_url(csrf),
        }
    }
    fn exchange_code(
        &self,
        code: AuthorizationCode,
    ) -> CodeTokenRequest<'_, BasicErrorResponse, BasicTokenResponse> {
        match self {
            Client::Revokable(client) => client.exchange_code(code),
            Client::NonRevokable(client) => client.exchange_code(code),
        }
    }
}

/// The internal authentication state
pub struct OAuthState {
    client: Client,
    sessions: RwLock<HashMap<Session, Login>>,
    /// Tokens for CSRF protection
    logins: Mutex<VecDeque<(CsrfToken, u64)>>,
    profile_url: String,
    profile_scope: String,
    profile_key: String,
}

impl fmt::Debug for OAuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthState")
            .field("sessions", &self.sessions)
            .field("logins", &self.logins)
            .field("profile_url", &self.profile_url)
            .finish()
    }
}

/// A unique session for a logged in user.
/// The session does not reuse the oauth token.
#[derive(Clone, Hash, PartialEq, Eq)]
struct Session([u8; Self::N]);
impl Session {
    const N: usize = 32;
    fn new() -> Self {
        Self(rand::rng().random())
    }
    fn from_cookie(cookie: &str) -> Result<Self> {
        let mut data = [0; Self::N + 8]; // has to be larger due to wrong estimates!
        let len = BASE64
            .decode_slice(cookie, &mut data)
            .map_err(|_| Error::Network)?;
        if len == Self::N {
            Ok(Self(data[..Self::N].try_into().unwrap()))
        } else {
            Err(Error::Network)
        }
    }
    fn to_cookie(&self) -> String {
        BASE64.encode(self.0)
    }
}
impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Session")
            .field(&BASE64.encode(self.0))
            .finish()
    }
}

async fn login_redirect(State(auth): State<Arc<OAuthState>>) -> impl IntoResponse {
    let (auth_url, csrf_token) = auth
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(auth.profile_scope.clone()))
        .url();

    // Store csrf token for later checks
    let mut logins = auth.logins.lock().unwrap();
    logins.push_front((csrf_token, unix_secs() + LOGIN_EXPIRE_SEC));
    logins.truncate(LOGIN_COUNT);

    Redirect::to(auth_url.as_ref())
}

async fn logout(
    State(auth): State<Arc<OAuthState>>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<impl IntoResponse> {
    if let Some(cookie) = cookies.get(COOKIE_NAME) {
        let session = Session::from_cookie(cookie)?;
        auth.sessions.write().unwrap().remove(&session);
    }
    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

#[tracing::instrument]
async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(auth): State<Arc<OAuthState>>,
) -> Result<impl IntoResponse> {
    {
        // Check CSRF
        let mut logins = auth.logins.lock().unwrap();
        let len = logins.len();
        logins.retain(|t| t.0.secret() != &query.state);
        if len == logins.len() {
            error!("CSRF token not found!");
            return Err(Error::Network); // CSRF token not found
        }
    }

    let http_client = reqwest::Client::builder()
        // Following redirects opens the client up to SSRF.
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // Get an auth token
    let token = auth
        .client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&http_client)
        .await?;

    info!("Token received");

    // Fetch user data
    let client = reqwest::Client::new();

    // For debugging fetched data
    // info!(
    //     "Received Data: {}",
    //     client
    //         .get(&*auth.profile_url)
    //         .bearer_auth(token.access_token().secret())
    //         .send()
    //         .await?
    //         .text()
    //         .await?
    // );

    let data: serde_json::Value = client
        .get(&*auth.profile_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json()
        .await?;

    // Parse user data (search for an id to show in the UI)
    let mut curr = &data;
    for part in auth.profile_key.split('.') {
        let Some(map) = curr.as_object() else {
            error!("Invalid user data, expected dict: {data:?}");
            return Err(Error::Network);
        };
        let Some(val) = map.get(part) else {
            error!("Invalid user data, key '{part:?}' not found: {data:?}");
            return Err(Error::Network);
        };
        curr = val;
    }
    let Some(id) = curr.as_str() else {
        error!("Invalid user id, expected string: {curr:?}");
        return Err(Error::Network);
    };

    warn!("Login: {id:?}");

    if let Client::Revokable(auth) = &auth.client {
        // Directly revoke token, not needed anymore
        info!("Revoke access");
        auth.revoke_token(token.access_token().into())
            .map_err(|_| Error::Network)?
            .request_async(&http_client)
            .await?;
    }

    // Create a new session filled with user data
    let session = Session::new();

    // Set cookie
    let cookie = format!(
        "{COOKIE_NAME}={}; SameSite=Lax; Path=/",
        session.to_cookie()
    );

    let login = Login {
        id: id.into(),
        expires: unix_secs() + SESSION_EXPIRE_SEC,
    };
    auth.sessions.write().unwrap().insert(session, login);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());
    Ok((headers, Redirect::to("/")))
}

impl<S> FromRequestParts<S> for Login
where
    Auth: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let auth = Auth::from_ref(state);
        let Auth::OAuth(auth) = auth else {
            return Ok(Login {
                id: String::new(),
                expires: 0,
            });
        };
        let sessions = &auth.sessions;

        let cookies = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|_| AuthRedirect)?;

        let cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;
        let session = Session::from_cookie(cookie).map_err(|_| AuthRedirect)?;
        let guard = sessions.read().unwrap();
        let login = guard.get(&session).ok_or(AuthRedirect)?;
        if unix_secs() > login.expires {
            sessions.write().unwrap().remove(&session);
            Err(AuthRedirect)
        } else {
            Ok(login.clone())
        }
    }
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/login").into_response()
    }
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod test {
    use super::Session;

    #[test]
    fn session() {
        let session = Session::new();
        let cookie = session.to_cookie();
        let parsed = Session::from_cookie(&cookie).unwrap();
        assert_eq!(session, parsed);
    }
}
