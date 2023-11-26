use std::path::PathBuf;
use std::{fs::File, net::SocketAddr};

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::db::AtomicDatabase;

// mod api;
mod db;
mod error;
mod isbn;
mod mail;
mod provider;
mod server;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Schiller Library Backend
#[derive(Parser)]
struct Args {
    /// Ip and port for the webserver
    host: SocketAddr,
    /// Externally visible domain of this webserver
    #[arg(long)]
    domain: Option<String>,
    /// Path to the oauth config
    #[arg(long)]
    auth: Option<PathBuf>,
    /// Directory for the static assets
    #[arg(short, long, default_value = "lib-view/build")]
    assets: PathBuf,
    /// Path to the database
    #[arg(short, long, default_value = "schillerbib.db")]
    db: PathBuf,
    /// Path to the users file
    #[arg(long, default_value = "users.txt")]
    user_file: PathBuf,
    /// CSV row delimiter for the users file
    #[arg(long, default_value_t = '|')]
    user_delimiter: char,
    /// Path to the TLS certificate
    #[arg(long)]
    cert: PathBuf,
    /// Path to the TLS key
    #[arg(long)]
    key: PathBuf,
}

#[tokio::main]
async fn main() {
    logging();

    let Args {
        host,
        domain,
        auth,
        assets,
        db,
        user_file,
        user_delimiter,
        cert,
        key,
    } = Args::parse();

    let auth = auth.map(|auth| {
        serde_json::from_reader(File::open(auth).expect("No OAuth Config found")).unwrap()
    });

    assert!(user_delimiter.is_ascii());

    let domain = domain.unwrap_or_else(|| host.to_string());

    let db = if db.exists() {
        AtomicDatabase::load(&db).unwrap()
    } else {
        AtomicDatabase::create(&db).unwrap()
    };

    server::start(
        host,
        &domain,
        auth,
        db,
        assets,
        user_file,
        user_delimiter as _,
        &cert,
        &key,
    )
    .await;
}

/// initialize tracing
fn logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
