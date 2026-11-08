use std::fs::File;
use std::io::{self, BufReader};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::error_handling::HandleErrorLayer;

use axum::http::StatusCode;

use hyper::Request;
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tower::{BoxError, ServiceBuilder};
use tower_http::compression::CompressionLayer;

use tower_http::trace::TraceLayer;
use tower_service::Service;
use tracing::{debug, error, info};

use crate::db::AtomicDatabase;
pub use crate::server::auth::Login;

mod auth;
use auth::Auth;
pub use auth::AuthConfig;

/// Start the backend server
pub async fn start(
    host: SocketAddr,
    domain: &str,
    db: AtomicDatabase,
    tls: Tls,
    auth: Option<AuthConfig>,
) {
    let tls = tls.load_config().expect("invalid TLS config");
    let auth = Auth::new(domain, auth);
    let app = app(auth.clone(), crate::web::router(db).build());

    debug!("Listening on https://{host}");

    let (_, r) = tokio::join!(auth::background(auth), serve(host, tls, app));
    r.unwrap();
}

fn app(auth: Auth, web: topcoat::router::Router) -> Router {
    Router::new()
        .nest("/auth", auth::routes(auth.clone()))
        .fallback_service(frontend(auth, web))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
}

fn frontend(auth: Auth, web: topcoat::router::Router) -> Router {
    let web = Arc::new(web);
    Router::new()
        .fallback(move |login: Login, request: Request<Body>| {
            let web = web.clone();
            async move {
                web.handle_with(request.map(topcoat::router::Body::new), (login,))
                    .await
                    .map(Body::new)
            }
        })
        .with_state(auth)
}

async fn handle_error(error: BoxError) -> StatusCode {
    if error.is::<tower::timeout::error::Elapsed>() {
        StatusCode::REQUEST_TIMEOUT
    } else {
        error!("Internal server error: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn serve(host: SocketAddr, tls: ServerConfig, app: Router) -> io::Result<()> {
    let acceptor = TlsAcceptor::from(Arc::new(tls));
    let listener = TcpListener::bind(&host).await?;

    loop {
        let (stream, peer) = match listener.accept().await {
            Ok(new) => new,
            Err(err) => {
                error!("client connection failed: {err:?}");
                continue;
            }
        };

        let acceptor = acceptor.clone();
        let app = app.clone();

        tokio::spawn(async move {
            let Ok(stream) = acceptor.accept(stream).await else {
                info!("tls handshake failed: {peer}");
                return;
            };
            let stream = TokioIo::new(stream);

            // Hyper has also its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `app` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                app.clone().call(request)
            });

            let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(stream, hyper_service)
                .await;

            if let Err(err) = ret {
                info!("serving failed {peer}: {err}");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::header::LOCATION;
    use tower::ServiceExt;

    fn oauth() -> Auth {
        Auth::new(
            "localhost:5000",
            Some(AuthConfig {
                client_id: "test".into(),
                client_secret: "test".into(),
                auth_url: "https://identity.example/authorize".into(),
                token_url: "https://identity.example/token".into(),
                revoke_url: None,
                profile_url: "https://identity.example/profile".into(),
                profile_scope: "profile".into(),
                profile_key: "id".into(),
            }),
        )
    }

    #[tokio::test]
    async fn frontend_requires_login_for_every_path_and_method() {
        let app = frontend(oauth(), topcoat::router::Router::builder().build());
        for (method, path) in [
            ("GET", "/"),
            ("GET", "/books"),
            ("GET", "/assets/test.js"),
            ("POST", "/runtime"),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(path)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
            assert_eq!(response.headers()[LOCATION], "/auth/login");
        }
    }

    #[tokio::test]
    async fn oauth_login_is_public() {
        let auth = oauth();
        let app = app(auth, topcoat::router::Router::builder().build());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/auth/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert!(
            response.headers()[LOCATION]
                .to_str()
                .unwrap()
                .starts_with("https://identity.example/authorize?")
        );
    }

    #[tokio::test]
    async fn frontend_without_oauth_reaches_topcoat() {
        let app = frontend(Auth::None, topcoat::router::Router::builder().build());
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}

/// TLS configuration
pub struct Tls {
    pub cert: PathBuf,
    pub key: PathBuf,
}

impl Tls {
    fn load_config(self) -> io::Result<ServerConfig> {
        use rustls_pemfile::{certs, pkcs8_private_keys};

        let certs = certs(&mut BufReader::new(File::open(self.cert)?)).collect::<Result<_, _>>()?;
        let key = pkcs8_private_keys(&mut BufReader::new(File::open(self.key)?))
            .next()
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))??;
        rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key.into())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
    }
}
