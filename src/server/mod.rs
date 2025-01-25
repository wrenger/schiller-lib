use std::fs::File;
use std::io::{self, BufReader};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::middleware::from_extractor_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use hyper::body::Incoming;
use hyper::Request;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use tower::{BoxError, ServiceBuilder, ServiceExt};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tower_service::Service;
use tracing::{debug, error, info};

use crate::db::AtomicDatabase;
use crate::provider;
use crate::server::auth::Login;

mod auth;
use auth::Auth;
pub use auth::AuthConfig;
mod api;
use api::Project;
pub use api::UserConfig;

/// Start the backend server
pub async fn start(
    host: SocketAddr,
    domain: &str,
    db: AtomicDatabase,
    assets: PathBuf,
    tls: Tls,
    auth: Option<AuthConfig>,
    user: Option<UserConfig>,
) {
    let tls = tls.load_config().expect("invalid TLS config");
    let auth = Auth::new(domain, auth);
    let project = Project::new(db, user, auth.clone());

    let app = Router::new()
        .nest("/auth", auth::routes(auth.clone()))
        .nest("/api", api::routes(project.clone()))
        .route(
            "/",
            get(static_index)
                .with_state(assets.clone())
                .layer(from_extractor_with_state::<Login, Auth>(auth.clone())),
        )
        .route(
            "/{*file}",
            get(static_assets)
                .with_state(assets)
                .layer(from_extractor_with_state::<Login, Auth>(auth.clone())),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    debug!("Listening on {host}");

    let (_, r) = tokio::join!(auth::background(auth), serve(host, tls, app));
    r.unwrap();
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
    let listener = TcpListener::bind(&host).await.unwrap();

    loop {
        let (stream, peer) = listener.accept().await?;
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

async fn static_index(State(dir): State<PathBuf>, req: Request<Body>) -> Response {
    ServeFile::new(dir.join("index.html"))
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}

async fn static_assets(
    State(dir): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> Response {
    if !file.contains('.') {
        ServeFile::new(dir.join(file).with_extension("html"))
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    } else {
        ServeDir::new(dir.clone())
            .oneshot(req)
            .await
            .unwrap()
            .into_response()
    }
}
