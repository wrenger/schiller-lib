use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::{FromRef, FromRequestParts, Query, State};
use axum::http::header::SET_COOKIE;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{async_trait, Json, RequestPartsExt, Router};
use axum_extra::headers::Cookie;
use axum_extra::TypedHeader;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hyper::{HeaderMap, StatusCode};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::error::{Error, Result};

static COOKIE_NAME: &str = "SESSION";
const SESSION_CHECK_SEC: u64 = 8 * 60 * 60; // 8h
const SESSION_EXPIRE_SEC: u64 = 3 * 24 * 60 * 60; // 3d

const LOGIN_COUNT: usize = 10;
const LOGIN_EXPIRE_SEC: u64 = 5 * 60;

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

/// The user data we'll get back from oauth.
///
/// E.g. Discord: https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Login {
    id: String,
    username: String,
    /// Custom data storing how long the session is valid
    #[serde(skip)]
    expires: u64,
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
            user_url,
        }) = config
        {
            let redirect = format!("https://{domain}/auth/authorized");
            let client = BasicClient::new(ClientId::new(client_id))
                .set_client_secret(ClientSecret::new(client_secret))
                .set_auth_uri(AuthUrl::new(auth_url).unwrap())
                .set_token_uri(TokenUrl::new(token_url).unwrap())
                .set_redirect_uri(RedirectUrl::new(redirect).unwrap());

            Self::OAuth(Arc::new(OAuthState {
                client,
                sessions: Default::default(),
                logins: Default::default(),
                user_url,
            }))
        } else {
            error!("SECURITY: Missing OAuth configuration!");
            Auth::None
        }
    }
}

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
    /// Endpoint for user data (requires a token)
    pub user_url: String,
}

/// The internal authentication state
pub struct OAuthState {
    client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
    sessions: RwLock<HashMap<Session, Login>>,
    /// Tokens for CSRF protection
    logins: Mutex<VecDeque<(CsrfToken, u64)>>,
    user_url: String,
}

impl fmt::Debug for OAuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthState")
            .field("sessions", &self.sessions)
            .field("logins", &self.logins)
            .field("user_url", &self.user_url)
            .finish()
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Session([u8; 32]);

impl Session {
    fn new() -> Self {
        let mut data = [0; 32];
        rand::thread_rng().fill_bytes(&mut data);
        Self(data)
    }
    fn from_cookie(cookie: &str) -> Result<Self> {
        let mut data = [0; 32 + 8]; // has to be larger due to fucked up estimates!
        let len = BASE64
            .decode_slice(cookie, &mut data)
            .map_err(|_| Error::Network)?;

        let mut ret = [0; 32];
        if len == ret.len() {
            let len = ret.len();
            ret.copy_from_slice(&data[..len]);
            Ok(Self(ret))
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
        .add_scope(Scope::new("identify".to_string()))
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
            return Err(Error::Network); // CSRF token not found
        }
    }

    let http_client = reqwest::Client::builder()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // Get an auth token
    let token = auth
        .client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&http_client)
        .await?;

    // Fetch user data from discord
    let client = reqwest::Client::new();
    let mut login: Login = client
        .get(&*auth.user_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json()
        .await?;

    login.expires = unix_secs() + SESSION_EXPIRE_SEC;

    // Create a new session filled with user data
    let session = Session::new();

    // Set cookie
    let cookie = format!(
        "{COOKIE_NAME}={}; SameSite=Lax; Path=/",
        session.to_cookie()
    );
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    auth.sessions.write().unwrap().insert(session, login);

    Ok((headers, Redirect::to("/")))
}

#[async_trait]
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
                username: String::new(),
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
