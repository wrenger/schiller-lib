use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

use crate::db::AtomicDatabase;
use crate::server::{Tls, UserConfig};

mod db;
mod error;
mod isbn;
mod mail;
mod provider;
mod server;
mod util;

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
    #[arg(short, long, default_value = "lib.json")]
    db: PathBuf,
    /// Path to the users file
    #[arg(long)]
    user_file: Option<PathBuf>,
    /// CSV row delimiter for the users file
    #[arg(long, default_value_t = ',')]
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
    util::logging();

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
    let delimiter = user_delimiter as u8;
    if let Some(user_file) = &user_file {
        assert!(user_file.exists(), "User file not found: {user_file:?}");
    }
    let user = user_file.map(|file| UserConfig { file, delimiter });

    let domain = domain.unwrap_or_else(|| host.to_string());

    let db = if db.exists() {
        AtomicDatabase::load(&db).unwrap()
    } else {
        AtomicDatabase::create(&db).unwrap()
    };

    let tls = Tls { cert, key };
    server::start(host, &domain, db, assets, tls, auth, user).await;
}
