topcoat::l10n::fluent::messages!(pub(crate) "./l10n/{locale}/**/*.ftl");

mod components;
mod db;
mod error;
mod fuzzy;
mod isbn;
mod mail;
mod provider;
mod server;
mod util;
mod web;

use std::env;
use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

use crate::db::{AtomicDatabase, UserConfig};
use crate::server::Tls;

/// Schiller Library Backend
#[derive(Parser)]
struct Args {
    /// IP and port (defaults to HOST/PORT or 127.0.0.1:3000; ignored by topcoat dev)
    host: Option<SocketAddr>,
    /// Public origin (https://library.example), or domain for HTTPS
    #[arg(long)]
    domain: Option<String>,
    /// Path to the oauth config
    #[arg(long)]
    auth: Option<PathBuf>,

    /// Path to the database
    #[arg(short, long, default_value = "test/demo.json")]
    db: PathBuf,
    /// Path to the users file
    #[arg(long)]
    user_file: Option<PathBuf>,
    /// CSV row delimiter for the users file
    #[arg(long, default_value_t = ',')]
    user_delimiter: char,
    /// Enable TLS with this certificate (ignored by topcoat dev)
    #[arg(long, requires = "key")]
    cert: Option<PathBuf>,
    /// TLS private key (requires --cert; ignored by topcoat dev)
    #[arg(long, requires = "cert")]
    key: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    util::logging();

    let Args {
        host,
        domain,
        auth,

        db,
        user_file,
        user_delimiter,
        cert,
        key,
    } = Args::parse();

    let dev = env::var_os("TOPCOAT_DEV_URL").is_some();
    let (host, tls) = transport(host, cert, key, dev);
    let (bind_host, port) = server::address(host).expect("Invalid HOST/PORT");
    let origin = public_origin(domain.as_deref(), &bind_host, port, tls.is_some());

    let auth = auth.map(|auth| {
        serde_json::from_reader(File::open(auth).expect("No OAuth Config found")).unwrap()
    });

    assert!(user_delimiter.is_ascii());
    let delimiter = user_delimiter as u8;
    if let Some(user_file) = &user_file {
        assert!(user_file.exists(), "User file not found: {user_file:?}");
    }
    let user = user_file.map(|file| UserConfig { file, delimiter });

    let db = if db.exists() {
        AtomicDatabase::load(&db, user.as_ref()).unwrap()
    } else {
        AtomicDatabase::create(&db).unwrap()
    };

    server::start(host, &origin, db, tls, auth)
        .await
        .expect("Server failed");
}

fn transport(
    host: Option<SocketAddr>,
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
    dev: bool,
) -> (Option<SocketAddr>, Option<Tls>) {
    if dev {
        (None, None)
    } else {
        (host, cert.zip(key).map(|(cert, key)| Tls { cert, key }))
    }
}

fn public_origin(domain: Option<&str>, host: &str, port: u16, tls: bool) -> String {
    if let Some(domain) = domain {
        if domain.starts_with("http://") || domain.starts_with("https://") {
            return domain.trim_end_matches('/').into();
        }
        return format!("https://{domain}");
    }

    let scheme = if tls { "https" } else { "http" };
    let host = if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]")
    } else {
        host.into()
    };
    format!("{scheme}://{host}:{port}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_is_opt_in_and_requires_both_paths() {
        let args = Args::try_parse_from(["schiller-lib"]).unwrap();
        assert!(args.host.is_none());
        assert!(args.cert.is_none());
        assert!(args.key.is_none());
        assert!(Args::try_parse_from(["schiller-lib", "--cert", "cert.pem"]).is_err());
        assert!(Args::try_parse_from(["schiller-lib", "--key", "key.pem"]).is_err());
        let args = Args::try_parse_from([
            "schiller-lib",
            "127.0.0.1:5000",
            "--cert",
            "cert.pem",
            "--key",
            "key.pem",
        ])
        .unwrap();
        let (host, tls) = transport(args.host, args.cert, args.key, false);
        assert_eq!(host.unwrap().port(), 5000);
        assert!(tls.is_some());
    }

    #[test]
    fn demo_database_is_default_and_can_be_overridden() {
        let args = Args::try_parse_from(["schiller-lib"]).unwrap();
        assert_eq!(args.db, PathBuf::from("test/demo.json"));
        let args = Args::try_parse_from(["schiller-lib", "--db", "lib.json"]).unwrap();
        assert_eq!(args.db, PathBuf::from("lib.json"));
    }

    #[test]
    fn dev_ignores_explicit_host_and_tls() {
        let (host, tls) = transport(
            Some("127.0.0.1:5000".parse().unwrap()),
            Some("missing.pem".into()),
            Some("missing.key".into()),
            true,
        );
        assert!(host.is_none());
        assert!(tls.is_none());
    }

    #[test]
    fn public_origin_matches_transport_and_accepts_proxy_origin() {
        assert_eq!(
            public_origin(None, "127.0.0.1", 3000, false),
            "http://127.0.0.1:3000"
        );
        assert_eq!(public_origin(None, "::1", 5000, true), "https://[::1]:5000");

        assert_eq!(
            public_origin(Some("library.example"), "127.0.0.1", 3000, false),
            "https://library.example"
        );
        assert_eq!(
            public_origin(Some("https://library.example/"), "127.0.0.1", 3000, false),
            "https://library.example"
        );
    }
}
