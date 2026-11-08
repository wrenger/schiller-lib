use std::fs::File;
use std::io::{self, BufReader};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinSet;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use topcoat::context::Cx;
use topcoat::router::response::IntoResponse;
use topcoat::router::{
    Body, LayerFn, LayerFuture, Listener, Next, Router, RouterBuilder, StatusCode,
};
use topcoat::runtime::RouterBuilderRuntimeExt;
use tracing::{Instrument, debug, error, info_span, warn};

use crate::db::AtomicDatabase;

mod auth;
use auth::Auth;
pub use auth::{AuthConfig, require_login};

/// Start the backend server. Topcoat dev always uses its HTTP transport.
pub async fn start(
    host: Option<SocketAddr>,
    origin: &str,
    db: AtomicDatabase,
    tls: Option<Tls>,
    auth: Option<AuthConfig>,
) -> io::Result<()> {
    let dev = std::env::var_os("TOPCOAT_DEV_URL").is_some();
    let host = if dev { None } else { host };
    let tls = if dev { None } else { tls };
    let auth = Auth::new(origin, auth);
    let app = app(auth.clone(), crate::web::router(db));

    // Stop the cleanup future when serving ends (including graceful shutdown).
    tokio::select! {
        result = serve(host, tls, app) => result,
        () = async {
            auth::background(auth).await;
            std::future::pending::<()>().await;
        } => unreachable!(),
    }
}

fn app(auth: Auth, web: RouterBuilder) -> Router {
    auth::routes(web.app_context(auth))
        .layer(LayerFn::new(None::<&str>, requests))
        .runtime()
        .build()
}

fn requests<'a>(cx: &'a Cx, body: Body, next: Next<'a>) -> LayerFuture<'a> {
    Box::pin(async move {
        let request = topcoat::router::request::parts(cx);
        let span = info_span!("request", method = %request.method, path = %request.uri.path());
        async move {
            let started = std::time::Instant::now();
            let response =
                match tokio::time::timeout(Duration::from_secs(10), next.run(cx, body)).await {
                    Ok(result) => result,
                    Err(_) => StatusCode::REQUEST_TIMEOUT.into_response(cx),
                };
            match &response {
                Ok(response) => debug!(status = %response.status(), elapsed = ?started.elapsed(), "Request handled"),
                Err(error) if error.is::<topcoat::router::error::RewriteError>()
                    || error.is::<topcoat::router::error::RedirectError>()
                    || error.is::<topcoat::router::error::SeeOther>() => {
                    debug!(elapsed = ?started.elapsed(), "Request returned to router");
                }
                Err(error) => {
                    error!(method = %request.method, path = %request.uri.path(), error = ?error, elapsed = ?started.elapsed(), "Request failed");
                }
            }
            response
        }
        .instrument(span)
        .await
    })
}

/// Resolve an explicit address or Topcoat's HOST/PORT defaults.
pub fn address(host: Option<SocketAddr>) -> io::Result<(String, u16)> {
    if let Some(host) = host {
        return Ok((host.ip().to_string(), host.port()));
    }
    let host = match std::env::var("HOST") {
        Ok(host) => host,
        Err(std::env::VarError::NotPresent) => "127.0.0.1".into(),
        Err(error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, error)),
    };
    let port = match std::env::var("PORT") {
        Ok(port) => port
            .parse()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?,
        Err(std::env::VarError::NotPresent) => 3000,
        Err(error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, error)),
    };
    Ok((host, port))
}

async fn serve(host: Option<SocketAddr>, tls: Option<Tls>, app: Router) -> io::Result<()> {
    if let Some(tls) = tls {
        let (host, port) = address(host)?;
        let listener = TcpListener::bind((host.as_str(), port)).await?;
        debug!(address = %listener.local_addr()?, "Listening with TLS");
        let listener = TlsListener {
            listener,
            acceptor: TlsAcceptor::from(Arc::new(tls.load_config()?)),
            handshakes: JoinSet::new(),
        };
        topcoat::serve(listener, app).await
    } else if let Some(host) = host {
        topcoat::serve(TcpListener::bind(host).await?, app).await
    } else {
        topcoat::start(app).await
    }
}

struct TlsListener {
    listener: TcpListener,
    acceptor: TlsAcceptor,
    handshakes: JoinSet<io::Result<(TlsStream<TcpStream>, SocketAddr)>>,
}

impl Listener for TlsListener {
    type Io = TlsStream<TcpStream>;

    async fn accept(&mut self) -> io::Result<(Self::Io, Option<SocketAddr>)> {
        loop {
            tokio::select! {
                accepted = self.listener.accept(), if self.handshakes.len() < 128 => {
                    let (stream, peer) = accepted?;
                    let acceptor = self.acceptor.clone();
                    // A stalled handshake must not block other clients or grow without bound.
                    self.handshakes.spawn(async move {
                        let stream = tokio::time::timeout(Duration::from_secs(10), acceptor.accept(stream))
                            .await.map_err(io::Error::other)??;
                        Ok((stream, peer))
                    });
                }
                result = self.handshakes.join_next(), if !self.handshakes.is_empty() => {
                    match result {
                        Some(Ok(Ok((stream, peer)))) => return Ok((stream, Some(peer))),
                        Some(Ok(Err(error))) => debug!(%error, "TLS handshake failed"),
                        Some(Err(error)) => warn!(%error, "TLS handshake task failed"),
                        None => {}
                    }
                }
            }
        }
    }

    fn tcp_addr(&self) -> Option<SocketAddr> {
        self.listener.local_addr().ok()
    }
}

/// TLS configuration, enabled only when both paths are supplied.
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

#[cfg(test)]
mod tests {
    use super::*;
    use topcoat::router::{request::Request, route};

    #[route(GET "/guarded")]
    async fn guarded(cx: &Cx) -> topcoat::Result<&'static str> {
        require_login(cx).await?;
        Ok("ok")
    }

    #[tokio::test]
    async fn no_auth_reaches_native_router() {
        let app = app(Auth::None, Router::builder().route(guarded));
        let response = app
            .handle(
                Request::builder()
                    .uri("/guarded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[topcoat::router::page("/runtime-test")]
    async fn runtime_page(cx: &Cx) -> topcoat::Result<impl topcoat::view::View> {
        require_login(cx).await?;
        Ok(topcoat::view::view! { <p>"Preloaded page"</p> })
    }

    #[route(GET "/rewrite-test")]
    async fn rewritten(_cx: &Cx) -> topcoat::Result<()> {
        Err(topcoat::router::error::rewrite("/guarded", Body::empty()).into())
    }

    #[tokio::test]
    async fn internal_rewrites_reach_the_router() {
        let app = app(
            Auth::None,
            Router::builder().route(rewritten).route(guarded),
        );
        let response = app
            .handle(
                Request::builder()
                    .uri("/rewrite-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = topcoat::router::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body.as_ref(), b"ok");
    }

    #[tokio::test]
    async fn runtime_preloads_return_page_frames() {
        let app = app(Auth::None, Router::builder().page(runtime_page));
        let response = app
            .handle(
                Request::builder()
                    .method("POST")
                    .uri("/runtime-test")
                    .header("x-topcoat-runtime", "true")
                    .header("content-type", "application/json")
                    .header("accept", "application/x-ndjson")
                    .body(Body::from(r#"{"signals":{}}"#))
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "application/x-ndjson");
        let body = topcoat::router::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let frames: Vec<serde_json::Value> = String::from_utf8(body.to_vec())
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert!(frames.iter().any(|frame| frame["t"] == "snapshot"
            && frame["html"].as_str().unwrap().contains("Preloaded page")));
        assert!(frames.iter().all(|frame| frame["t"] != "error"));
    }

    #[route(GET "/failure-test")]
    async fn failure(_cx: &Cx) -> topcoat::Result<()> {
        Err(topcoat::Error::msg("Underlying database failure").context("Could not load books"))
    }

    #[tokio::test]
    async fn handler_errors_log_context_without_exposing_details() {
        use std::io::Write;
        use std::sync::Mutex;

        struct LogWriter(Arc<Mutex<Vec<u8>>>);
        impl Write for LogWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.0.lock().unwrap().write(bytes)
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let logs = Arc::new(Mutex::new(Vec::new()));
        let writer = logs.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_ansi(false)
            .with_writer(move || LogWriter(writer.clone()))
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);
        let app = app(Auth::None, Router::builder().route(failure));
        let response = app
            .handle(
                Request::builder()
                    .uri("/failure-test?secret=not-for-logs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = topcoat::router::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body.as_ref(), b"internal server error");
        let logs = String::from_utf8(logs.lock().unwrap().clone()).unwrap();
        assert!(logs.contains("Request failed"));
        assert!(logs.contains("Could not load books"));
        assert!(logs.contains("Underlying database failure"));
        assert!(logs.contains("/failure-test"));
        assert!(logs.contains("GET"));
        assert!(!logs.contains("not-for-logs"));
    }

    #[test]
    fn explicit_address_is_preserved() {
        assert_eq!(
            address(Some("[::1]:5000".parse().unwrap())).unwrap(),
            ("::1".into(), 5000)
        );
    }
}
