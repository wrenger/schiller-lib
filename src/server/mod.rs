use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::middleware::from_extractor_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use hyper::{Body, Request};
use tower::{BoxError, ServiceBuilder, ServiceExt};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{debug, error};

use crate::db;
use crate::provider;
use crate::server::auth::Login;

mod auth;
use auth::Auth;
pub use auth::AuthConfig;
mod api;
use api::Project;

pub async fn start(
    host: SocketAddr,
    domain: &str,
    auth: Option<AuthConfig>,
    db: db::Database,
    dir: PathBuf,
    user_file: PathBuf,
    user_delimiter: u8,
    cert: &std::path::Path,
    key: &std::path::Path,
) {
    let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();

    let auth = Auth::new(domain, auth);
    let project = Project::new(db, user_file, user_delimiter, auth.clone());

    let app = Router::new()
        .nest("/auth", auth::routes(auth.clone()))
        .nest("/api", api::routes(project.clone()))
        .route(
            "/",
            get(static_index)
                .with_state(dir.clone())
                .layer(from_extractor_with_state::<Login, Auth>(auth.clone())),
        )
        .route(
            "/*file",
            get(static_assets)
                .with_state(dir)
                .layer(from_extractor_with_state::<Login, Auth>(auth.clone())),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        error!("Internal server error: {error}");
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    debug!("Listening on {host}");

    let (_, r) = tokio::join!(
        auth::background(auth),
        axum_server::bind_rustls(host, config).serve(app.into_make_service())
    );
    r.unwrap();
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
