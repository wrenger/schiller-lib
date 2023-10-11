use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use db::Database;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

#[derive(Parser)]
struct Args {
    host: SocketAddr,
    #[arg(short, long, default_value = "schillerbib.db")]
    db: PathBuf,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let Args { host, db } = Args::parse();

    let db = if db.exists() {
        Database::open(db.into()).unwrap().0
    } else {
        Database::create(db.into()).unwrap()
    };

    server::start(host, db).await;
}
