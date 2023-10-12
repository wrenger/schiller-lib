use std::sync::Arc;

use crate::error::{Error, Result};

use axum::extract::FromRef;
use axum::extract::{
    rejection::TypedHeaderRejectionReason, FromRequestParts, Query, State, TypedHeader,
};
use axum::http::{header, header::SET_COOKIE, request::Parts};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{async_trait, Json, Router};
use axum::{headers::Cookie, RequestPartsExt};

use async_session::{MemoryStore, Session, SessionStore};
use hyper::{HeaderMap, StatusCode};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

static COOKIE_NAME: &str = "SESSION";

pub fn routes(auth: Auth) -> Router {
    Router::new()
        .route("/login", get(login_redirect))
        .route("/authorized", get(login_authorized))
        .route("/logout", get(logout))
        .fallback(|| async { (StatusCode::NOT_FOUND, Json(Error::NothingFound)) })
        .with_state(auth)
}

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Login {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
}

#[derive(Debug, Clone)]
pub struct Auth {
    client: BasicClient,
    sessions: MemoryStore,
    user_url: Arc<String>,
}

/// Configuration for OAuth
/// - client_id: REPLACE_ME
/// - client_secret: REPLACE_ME
/// - auth_url: Login page from the OAuth server
///     - https://discord.com/api/oauth2/authorize?response_type=code
/// - token_url: Convert the login code to a token
///     - https://discord.com/api/oauth2/token
/// - user_url: Endpoint for user data (requires a token)
///     - https://discordapp.com/api/users/@me
#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    user_url: String,
}
impl Auth {
    /// domain: Public domain of the webserver used for redirection
    pub fn new(domain: &str, config: AuthConfig) -> Self {
        let redirect = format!("https://{domain}/auth/authorized");
        let AuthConfig {
            client_id,
            client_secret,
            auth_url,
            token_url,
            user_url,
        } = config;

        let oauth = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(auth_url).unwrap(),
            Some(TokenUrl::new(token_url).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect).unwrap());

        Self {
            client: oauth,
            sessions: MemoryStore::new(),
            user_url: Arc::new(user_url),
        }
    }
}

async fn login_redirect(State(auth): State<Auth>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = auth
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();
    Redirect::to(auth_url.as_ref())
}

async fn logout(
    State(auth): State<Auth>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<impl IntoResponse> {
    if let Some(cookie) = cookies.get(COOKIE_NAME) {
        if let Some(session) = auth.sessions.load_session(cookie.into()).await? {
            auth.sessions.destroy_session(session).await?;
        }
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
    State(auth): State<Auth>,
) -> Result<impl IntoResponse> {
    // Get an auth token
    let token = auth
        .client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await?;

    // Fetch user data from discord
    let client = reqwest::Client::new();
    let user_data: Login = client
        .get(&*auth.user_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json::<Login>()
        .await?;

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("login", user_data)?;

    // Store session and get corresponding cookie
    let cookie = auth
        .sessions
        .store_session(session)
        .await?
        .ok_or(Error::NothingFound)?;

    // Set cookie
    let mut headers = HeaderMap::new();
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("/")))
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/login").into_response()
    }
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
        let cookies =
            parts
                .extract::<TypedHeader<Cookie>>()
                .await
                .map_err(|e| match *e.name() {
                    header::COOKIE => match e.reason() {
                        TypedHeaderRejectionReason::Missing => AuthRedirect,
                        _ => panic!("unexpected error getting Cookie header(s): {e}"),
                    },
                    _ => panic!("unexpected error getting cookies: {e}"),
                })?;

        let session = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;
        let auth = Auth::from_ref(&state);
        let login = auth
            .sessions
            .load_session(session.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?
            .get::<Login>("login")
            .ok_or(AuthRedirect)?;

        Ok(login)
    }
}
