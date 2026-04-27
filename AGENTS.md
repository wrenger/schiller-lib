# Agent Instructions for Schiller-Lib

This document contains high-signal, repo-specific guidance for AI agents working in this repository.

## Architecture & Boundaries
- **Monorepo:** Contains both the backend (`.` root directory) and frontend (`lib-view/`).
- **Backend:** Rust, Axum, SQLite (optional feature). Exposes a REST API and handles data.
- **Frontend:** Svelte 4, SvelteKit, TypeScript, Shadcn, Tailwind. Lives entirely in `lib-view/`.
- **Database:** Development database is stored as a single local JSON file (`test/lib.json`).

## Build & Generation Constraints
- **CRITICAL ORDERING:** You MUST build the backend *before* the frontend. The Rust backend uses `gluer` to generate the API client bindings that the SvelteKit frontend depends on.
- **Backend build:** `cargo build`
- **Frontend build:** `cd lib-view && bun install && bun run build`

## Development Workflow
- **Environment:** The project uses Nix (`flake.nix`). If Nix/direnv is active, `bun` and `rust` are provided.
- **Certificates:** The dev server requires TLS certificates. Generate them first:
  ```bash
  ./test/cert/gen.sh
  ```
- **Starting the Server:** Use the following command to start the backend with the test database and certs:
  ```bash
  cargo run -- 127.0.0.1:5000 -d test/lib.json --cert test/cert/cert.pem --key test/cert/key.pem
  ```
  *(Note: A new database is created if `test/lib.json` does not exist.)*
- **OAuth config:** The server requires an `auth.json` in the root for OAuth2. See `README.md` for the format.

## Verification & Testing
- **Backend Tests:** Run `cargo test` in the root.
- **Frontend Checks:** `cd lib-view && bun run check` (typecheck) and `bun run lint` (prettier).
