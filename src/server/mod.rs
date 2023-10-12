use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::middleware::from_extractor_with_state;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower::{BoxError, ServiceBuilder};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::debug;

use crate::db;
use crate::provider;

mod auth;
use auth::Auth;
pub use auth::AuthConfig;
mod api;
use api::Project;

use self::auth::Login;

pub async fn start(
    host: SocketAddr,
    domain: &str,
    auth: AuthConfig,
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
        // static files
        .fallback_service(
            Router::new()
                .fallback_service(ServeDir::new(dir))
                // requires authorization
                .layer(from_extractor_with_state::<Login, Auth>(auth)),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    debug!("Listening on {host}");

    axum_server::bind_rustls(host, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
