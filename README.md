# SchillerLib

The Schiller library software:
A small, simple, and intuitive library program for school libraries.

## Rework

This project is currently in a highly unstable stage due to a fundamental rework! A test Version can be seen [here](https://nils.wrenger.net).

### Frontend

- [x] Multi Language Support
- [x] Server Requests -> Error Modal
- [x] Borrowed Page
- [x] Infos Page
- [x] Show total amount of Infos
- [x] Reminders Modal
- [x] Borrow/Reserve/Extend Modal
- [x] Books/User List with Search and Adding Button -> Set Up a Grid and Generalize
- [x] Page Views including editing, deleting,...

### Backend

- [x] Static assets
- [x] TLS
- [x] About, Statistics, ...
- [x] Books
- [x] Users
- [x] Categories
- [x] Lending, Reserving, Overdues
- [x] Updating User Roles
- [x] Fetching Book Data
- [x] E-Mails
- [x] OAuth & Logins

### Download

The latest builds can be downloaded from the [releases page](https://github.com/wrenger/schiller-lib/releases/latest).


## Usage

The webserver uses [Rust](https://www.rust-lang.org/learn/get-started), which has to be installed first.

The frontend uses [Svelte](https://svelte.dev), Typescript and [Skeleton](https://www.skeleton.dev/), which also have to be installed first.
As a Package manager we would recommend using [bun](https://bun.sh/).

Using cargo, the project can be built and executed:

```sh
# generate TLS certificates with OpenSSL
./test/data/cert/gen.sh
# prepare the auth.json with client_id, client_secret, auth_url, token_url, user_url
# build the frontend in the lib-view directory
bun run build
# start the webserver on port 5000
cargo run -- 127.0.0.1:5000 -d test/data/lib.db --cert test/data/cert/cert.pem --key test/data/cert/key.pem --user-file test/data/users.txt --auth test/data/auth.json --assets lib-view/build
```

> A new database is created if the provided path to the database is non-existent.

## Architecture

This application follows the 3-tier principle.

- **UI Layer:** This is implemented in Svelte and TypeScript.
- **Application Layer:** This is implemented in Rust using the [axum](https://github.com/tokio-rs/axum) webserver.
It contains the business logic; most of the computation is done in this layer.
- **Database Layer:** The SQLite database that stores the persistent data specific to a project.

### UI Layer

The [UI](lib-view) is developed in Svelte and TypeScript using
Skeleton as a very powerful UI Framework.

This layer is also responsible for internationalization
([locales](lib-view/src/lib/i18n/locales/)).
Currently, there are only two languages supported (English and German).
Contributions for new languages or improved translations are very welcome.

### Application Layer

This layer is implemented in Rust ([src](src)) and exposes a REST API ([src/server](src/server/mod.rs)) that is used by the UI.

It is responsible for consistency checks and business logic.
This layer also manages the user logins and database connections, fetches data from external sources, and sends E-Mail notifications.

### Database Layer

The [SQLite](https://sqlite.org/index.html) database has the following schema:

<img src="images/sbv_db.svg" alt="Database Schema" width=400 />

It contains any project-specific data and settings and can be distributed as such.

## Package & Distribute

> **TODO:** Update Description

After building the GDNative library, the project can be exported within the Godot editor.

> See https://docs.godotengine.org/en/stable/getting_started/workflow/export/index.html

Alternatively, this can be done from the command line:
```bash
# linux
mkdir export/x11
path/to/godot --export x11 export/x11/schiller-lib.x86_64
# windows
mkdir export/win
path/to/godot --export win export/win/schiller-lib.exe
```
