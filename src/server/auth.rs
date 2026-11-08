use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, AuthorizationRequest, ClientId, ClientSecret, CodeTokenRequest,
    CsrfToken, EndpointNotSet, EndpointSet, RedirectUrl, RevocationUrl, Scope, TokenResponse,
    TokenUrl,
};
use serde::{Deserialize, Serialize};
use topcoat::{
    context::{Cx, app_context, memoize},
    cookie::{Cookie, Cookies, RouterBuilderCookieExt, SameSite, cookies, time},
    router::{
        RouterBuilder,
        error::{SeeOther, bad_request, redirect, see_other},
        href, parse_query_params, route,
    },
    session::{self, RouterBuilderSessionExt, SessionConfig, TokenHash},
};
use tracing::{debug, error, info, warn};

const CSRF_COOKIE: &str = "OAUTH_CSRF";
const SESSION_CHECK_SEC: u64 = 8 * 60 * 60; // 8h
const SESSION_EXPIRE_SEC: u64 = 2 * 24 * 60 * 60; // 2d

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
pub fn routes(builder: RouterBuilder) -> RouterBuilder {
    builder
        .route(login_redirect)
        .route(login_authorized)
        .route(logout)
        .sessions(
            SessionConfig::builder()
                .lifetime(Duration::from_secs(SESSION_EXPIRE_SEC))
                .build(),
        )
        .cookies()
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
            });
        }
    }
}

/// The user data for a successful login.
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
    /// This requires the public origin of the webserver used for redirections
    pub fn new(origin: &str, config: Option<AuthConfig>) -> Self {
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
            let redirect = format!("{}/auth/authorized", origin.trim_end_matches('/'));
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
    sessions: RwLock<HashMap<TokenHash, Login>>,
    profile_url: String,
    profile_scope: String,
    profile_key: String,
}

impl fmt::Debug for OAuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthState")
            .field("sessions", &self.sessions)
            .field("profile_url", &self.profile_url)
            .field("profile_scope", &self.profile_scope)
            .field("profile_key", &self.profile_key)
            .finish_non_exhaustive()
    }
}

#[route(GET "/auth/login")]
async fn login_redirect(cx: &Cx) -> topcoat::Result<SeeOther> {
    let Auth::OAuth(auth) = app_context(cx) else {
        return Ok(see_other(href!(crate::web::_index::page).resolve(cx)));
    };
    let (auth_url, csrf_token) = auth
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(auth.profile_scope.clone()))
        .url();
    cookies(cx).add(
        Cookie::build((CSRF_COOKIE, csrf_token.secret().to_owned()))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .max_age(time::Duration::seconds(LOGIN_EXPIRE_SEC as i64))
            .build(),
    );
    Ok(see_other(auth_url.as_str()))
}

#[route(POST "/auth/logout")]
async fn logout(cx: &Cx) -> topcoat::Result<SeeOther> {
    let hash = session::stop(cx).await?;
    if let (Auth::OAuth(auth), Some(hash)) = (app_context::<Auth>(cx), hash) {
        auth.sessions.write().unwrap().remove(&hash);
    }
    Ok(see_other(href!(crate::web::_index::page).resolve(cx)))
}

#[derive(Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

#[route(GET "/auth/authorized")]
async fn login_authorized(cx: &Cx) -> topcoat::Result<()> {
    let Auth::OAuth(auth) = app_context(cx) else {
        return Err(redirect(href!(crate::web::_index::page).resolve(cx)).into());
    };
    let query: AuthRequest =
        parse_query_params(cx).map_err(|_| bad_request("Invalid OAuth callback"))?;
    let csrf = cookies(cx).get(CSRF_COOKIE);
    if query.state.is_empty()
        || csrf.as_ref().map(|cookie| cookie.value()) != Some(query.state.as_str())
    {
        return Err(bad_request("Invalid OAuth state").into());
    }
    cookies(cx).remove(
        Cookie::build((CSRF_COOKIE, ""))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .path("/")
            .build(),
    );

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
    let data: serde_json::Value = http_client
        .get(&*auth.profile_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json()
        .await?;
    debug!("User data: {data:?}");

    // Parse user data (search for an id to show in the UI)
    let mut curr = &data;
    for part in auth.profile_key.split('.') {
        let Some(map) = curr.as_object() else {
            error!("Invalid user data, expected dict: {data:?}");
            return Err(topcoat::Error::msg("Invalid OAuth profile"));
        };
        let Some(val) = map.get(part) else {
            error!("Invalid user data, key '{part:?}' not found: {data:?}");
            return Err(topcoat::Error::msg("Invalid OAuth profile"));
        };
        curr = val;
    }
    let Some(id) = curr.as_str() else {
        error!("Invalid user id, expected string: {curr:?}");
        return Err(topcoat::Error::msg("Invalid OAuth profile"));
    };

    warn!("Login: {id:?}");

    if let Client::Revokable(auth) = &auth.client {
        // Directly revoke token, not needed anymore
        info!("Revoke access");
        if let Err(e) = auth
            .revoke_token(token.access_token().into())
            .map_err(|_| topcoat::Error::msg("OAuth revocation unavailable"))?
            .request_async(&http_client)
            .await
        {
            warn!("Failed to revoke user token {id:?}: {e}");
        }
    }

    let session = session::start(cx).await?;
    let login = Login {
        id: id.into(),
        expires: session
            .expires_at
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    auth.sessions
        .write()
        .unwrap()
        .insert(session.token_hash, login);
    Err(redirect(href!(crate::web::_index::page).resolve(cx)).into())
}

#[memoize]
async fn current_login(cx: &Cx) -> topcoat::Result<Option<Login>> {
    let Auth::OAuth(auth) = app_context(cx) else {
        return Ok(Some(Login::default()));
    };
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    let login = auth.sessions.read().unwrap().get(&hash).cloned();
    match login {
        Some(login) if login.expires > unix_secs() => Ok(Some(login)),
        Some(_) => {
            auth.sessions.write().unwrap().remove(&hash);
            Ok(None)
        }
        None => Ok(None),
    }
}

pub async fn require_login(cx: &Cx) -> topcoat::Result<Login> {
    current_login(cx)
        .await
        .as_ref()
        .map_err(Clone::clone)?
        .clone()
        .ok_or_else(|| redirect(href!(login_redirect).resolve(cx)).into())
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use topcoat::router::{Method, StatusCode, header, request::Request, response::Response};
    use topcoat::{
        router::{Body, ModuleRouterBuilder, Router},
        session::Token,
    };

    #[route(GET "/guard-test")]
    async fn guarded(cx: &Cx) -> topcoat::Result<&'static str> {
        require_login(cx).await?;
        Ok("ok")
    }

    fn oauth() -> Auth {
        Auth::new(
            "https://example.com",
            Some(AuthConfig {
                client_id: "client".into(),
                client_secret: "secret".into(),
                auth_url: "https://provider.example/authorize".into(),
                token_url: "https://provider.example/token".into(),
                revoke_url: None,
                profile_url: "https://provider.example/profile".into(),
                profile_scope: "profile".into(),
                profile_key: "id".into(),
            }),
        )
    }

    fn router(auth: Auth) -> Router {
        let builder: RouterBuilder =
            ModuleRouterBuilder::new(concat!(env!("CARGO_CRATE_NAME"), "::web"))
                .page(crate::web::_index::page)
                .into();
        routes(builder.app_context(auth).route(guarded)).build()
    }

    async fn request(router: &Router, method: Method, uri: &str, cookie: Option<&str>) -> Response {
        let mut builder = Request::builder().method(method).uri(uri);
        if let Some(cookie) = cookie {
            builder = builder.header(header::COOKIE, cookie);
        }
        router.handle(builder.body(Body::empty()).unwrap()).await
    }

    #[tokio::test]
    async fn public_login_and_invalid_csrf() {
        let router = router(oauth());
        let response = request(&router, Method::GET, "/auth/login", None).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let location = response
            .headers()
            .get(header::LOCATION)
            .unwrap()
            .to_str()
            .unwrap();
        let url = reqwest::Url::parse(location).unwrap();
        assert!(
            url.query_pairs().any(|(key, value)| key == "redirect_uri"
                && value == "https://example.com/auth/authorized")
        );
        let cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(
            cookie.contains("Secure")
                && cookie.contains("HttpOnly")
                && cookie.contains("SameSite=Lax")
        );
        let response = request(
            &router,
            Method::GET,
            "/auth/authorized?code=abc&state=wrong",
            Some("OAUTH_CSRF=other"),
        )
        .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        for (uri, cookie) in [
            ("/auth/authorized?code=abc&state=missing", None),
            ("/auth/authorized?code=abc&state=", Some("OAUTH_CSRF=")),
        ] {
            assert_eq!(
                request(&router, Method::GET, uri, cookie).await.status(),
                StatusCode::BAD_REQUEST
            );
        }
    }

    #[tokio::test]
    async fn sessions_and_logout() {
        let auth = oauth();
        let Auth::OAuth(state) = &auth else {
            unreachable!()
        };
        let token = Token::random();
        let cookie = format!("__Host-session={}", token.encode());
        state.sessions.write().unwrap().insert(
            token.hash(),
            Login {
                id: "user".into(),
                expires: unix_secs() + 60,
            },
        );
        let router = router(auth.clone());
        for cookie in [None, Some("__Host-session=invalid")] {
            assert_eq!(
                request(&router, Method::GET, "/guard-test", cookie)
                    .await
                    .status(),
                StatusCode::TEMPORARY_REDIRECT
            );
        }
        assert_eq!(
            request(&router, Method::GET, "/guard-test", Some(&cookie))
                .await
                .status(),
            StatusCode::OK
        );
        let response = request(&router, Method::GET, "/auth/logout", Some(&cookie)).await;
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        let response = router
            .handle(
                Request::builder()
                    .method(Method::POST)
                    .uri("/auth/logout")
                    .header(header::HOST, "example.com")
                    .header(header::ORIGIN, "https://evil.example")
                    .header(header::COOKIE, &cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            request(&router, Method::GET, "/guard-test", Some(&cookie))
                .await
                .status(),
            StatusCode::OK
        );
        let response = request(&router, Method::POST, "/auth/logout", Some(&cookie)).await;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert!(
            response
                .headers()
                .get_all(header::SET_COOKIE)
                .iter()
                .any(|v| v.to_str().unwrap().contains("__Host-session="))
        );
        assert_eq!(
            request(&router, Method::GET, "/guard-test", Some(&cookie))
                .await
                .status(),
            StatusCode::TEMPORARY_REDIRECT
        );
        state.sessions.write().unwrap().insert(
            token.hash(),
            Login {
                id: "user".into(),
                expires: unix_secs(),
            },
        );
        assert_eq!(
            request(&router, Method::GET, "/guard-test", Some(&cookie))
                .await
                .status(),
            StatusCode::TEMPORARY_REDIRECT
        );
        assert!(!state.sessions.read().unwrap().contains_key(&token.hash()));
    }

    #[tokio::test]
    async fn no_auth_bypasses_guard() {
        let router = router(Auth::None);
        assert_eq!(
            request(&router, Method::GET, "/guard-test", None)
                .await
                .status(),
            StatusCode::OK
        );
    }
}
