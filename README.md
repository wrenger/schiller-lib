# The Schiller Library Software

A small, simple, and intuitive library program for school libraries.

The latest builds can be downloaded from the [releases page](https://github.com/wrenger/schiller-lib/releases/latest).


## Usage

The webserver uses [Rust](https://www.rust-lang.org/learn/get-started), which has to be installed first.

The front end uses [Svelte](https://svelte.dev) and [Shadcn](https://www.shadcn-svelte.com/), which must be installed.
We would recommend using [Bun](https://bun.sh/) as the package manager.

The project can be built and executed with the following commands:

```sh
# generate TLS certificates with OpenSSL
./test/cert/gen.sh
# build the front end
cd lib-view
bun i
bun run build
cd ..
# start the web server
cargo run -- 127.0.0.1:5000 -d test/lib.json --cert test/cert/cert.pem --key test/cert/key.pem
```

> A new database is created if the provided file (`test/lib.json`) is non-existent.


## OAuth2

The web server uses OAuth2 for user authentication.

For this, a configuration file (`auth.json`) must be provided:

```json
{
    "client_id": "...",
    "client_secret": "...",
    "auth_url": "https://example.com/api/oauth2/authorize?response_type=code",
    "token_url": "https://example.com/api/oauth2/token",
    "user_url": "https://example.com/api/users/@me"
}
```


## Architecture

This application is built as a classical web server, with a single-page web UI communicating over a REST API with the server.

- **UI Layer:** This is implemented in Svelte and TypeScript.
- **Application Layer:** This is implemented in Rust using the [Axum](https://github.com/tokio-rs/axum) webserver.
It contains the business and handles the data.

### UI Layer

The [UI](lib-view) is developed in Svelte and TypeScript using Skeleton as a UI Framework.

This layer is also responsible for internationalization ([locales](lib-view/src/lib/i18n/locales/)).
Currently, there are only two languages supported (English and German).
Contributions for new languages or improved translations are very welcome.

### Application Layer

This layer is implemented in Rust ([src](src)) and exposes a REST API ([src/server](src/server/mod.rs)) that is used by the UI.

It is responsible for consistency checks and business logic.
This layer also manages the user logins and data storage, fetches data from external sources, and sends E-Mail notifications.

The entire project is stored and loaded from a single JSON file.


## Package & Distribute

For distribution, the following parts have to be built or configured.

- **Back End:** `target/release/schiller-lib` (using `cargo build -r`)
- **Front End:** `lib-view/build` (using `bun i && bun run build`)
- **SSL Certificates:** `test/cert/(cert|key).pem` (using `test/cert/gen.sh` or your own certificates)
- **OAuth Config:** `auth.json` (see [OAuth2](#oauth2))
- **Database:** `lib.json` (optional, if not provided, a new database is created)
