# The Schiller Library Software

A small, simple, and intuitive library program for school libraries.

The latest builds can be downloaded from the [releases page](https://github.com/wrenger/schiller-lib/releases/latest).


## Usage

The webserver uses [Rust](https://www.rust-lang.org/learn/get-started), which has to be installed first.

The UI and server use [Topcoat](https://github.com/tokio-rs/topcoat). Build the backend before any frontend build or checks.

```sh
cargo build
# Development: builds/bundles assets, serves HTTP, and reloads on source changes
topcoat dev
```

Without an explicit address, binding uses `HOST` and `PORT`, defaulting to `127.0.0.1:3000`. The positional address remains available for normal runs. `topcoat dev` always uses its own `HOST`/`PORT` and plain HTTP, ignoring explicit addresses and TLS paths.

For a bundled normal build, run `topcoat asset bundle`, then start the binary:

```sh
# HTTP (for example, behind an HTTPS reverse proxy)
./target/debug/schiller-lib -d test/lib.json
# Optional direct TLS: both --cert and --key are required
./test/cert/gen.sh
./target/debug/schiller-lib 127.0.0.1:5000 -d test/lib.json --cert test/cert/cert.pem --key test/cert/key.pem
```

The default database is `test/demo.json`, including the demo book used by the books preview. Override it with `--db path/to/library.json`. Loading and saving rewrites the database, so the development watcher ignores `test/demo.json` and `lib.json`; keep custom runtime databases in ignored paths as well. The demo database remains tracked by Git.

> A new database is created if the provided file (`test/lib.json`) is non-existent.

Handler failures log the request method, path, and error chain on the server. Browser responses retain generic 500 messages to avoid exposing internal details.


## OAuth2

The web server uses OAuth2 for user authentication.

Enable authentication with `--auth auth.json`. Without this option, authentication is disabled; do not expose that mode publicly. Set `--domain https://library.example` to the browser-facing origin when behind a proxy. A bare domain retains the previous HTTPS behavior; without `--domain`, the callback origin follows the bind address and HTTP/TLS mode.

Sessions use Topcoat's secure, HttpOnly, SameSite=Lax cookies and server-side token hashes. Configure browser-facing HTTPS for authenticated use; HTTP development does not automatically weaken cookies. Existing sessions are invalidated by this migration. Logout is `POST /auth/logout`, not GET.

Example configuration:

```json
{
    "client_id": "...",
    "client_secret": "...",
    "auth_url": "https://example.com/api/oauth2/authorize?response_type=code",
    "token_url": "https://example.com/api/oauth2/token",
    "profile_url": "https://example.com/api/users/@me",
    "profile_scope": "identify",
    "profile_key": "id"
}
```


## Architecture

The application serves server-rendered Topcoat pages from [src/web](src/web), with shared UI components in [src/components](src/components) and translations in [src/l10n](src/l10n).

### Application Layer

The Rust [server](src/server/mod.rs) serves a native Topcoat router over HTTP or an optional TLS listener. OAuth routes use Topcoat cookies and sessions; protected handlers call an authentication helper explicitly.

It is responsible for consistency checks and business logic.
This layer also manages the user logins and data storage, fetches data from external sources, and sends E-Mail notifications.

The entire project is stored and loaded from a single JSON file.


## Package & Distribute

For distribution, the following parts have to be built or configured.

- **Application and assets:** build using `topcoat asset bundle --release`; distribute the executable together with its generated `assets` directory.
- **TLS Certificates:** optional when terminating HTTPS at a reverse proxy; otherwise supply `--cert` and `--key` (see `test/cert/gen.sh`).
- **OAuth Config:** `auth.json`, enabled with `--auth` (see [OAuth2](#oauth2))
- **Database:** use `--db lib.json` for your own library; the default is `test/demo.json`. A missing database file is created if its parent directory exists.
