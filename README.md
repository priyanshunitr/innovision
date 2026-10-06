# Innovision

A Rust backend built with Axum. The current API exposes a greeting and a health endpoint.

## Start the backend

Install Rust with Cargo (a toolchain supporting Rust edition 2024), then run these commands from the repository root in PowerShell:

```powershell
cd backend
Copy-Item .env.example .env
# Edit .env with your local database URL before starting.
cargo run --locked
```

Copy the example only during initial setup to avoid overwriting an existing `.env`.
Run Cargo from `backend` so dotenvy can find `backend/.env`. It searches the current directory and its parents. Existing shell environment variables take precedence over values in `.env`.

| Variable | Default | Purpose |
| --- | --- | --- |
| `PORT` | `3000` | TCP port, from 1 to 65535. The server binds to `0.0.0.0`. |
| `DATABASE_URL` | Required | PostgreSQL connection URL, for example `postgres://postgres:postgres@localhost:5432/innovision`. |
| `RUST_LOG` | `info` | Logging filter, for example `debug` or `info,backend=debug`. |

`DATABASE_URL` is read and its format is validated at startup. The backend does not connect to PostgreSQL yet, so a running database is not required for these endpoints. Missing or invalid configuration produces a startup error. Database credentials are not logged.

`.env` and `.env.*` files are ignored by Git; `.env.example` remains tracked. Keep actual credentials in your local `.env` or deployment environment.

You can also start without a `.env` by setting environment variables:

```powershell
cd backend
$env:PORT = "3001"
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/innovision"
$env:RUST_LOG = "info,backend=debug"
cargo run --locked
```

In another PowerShell terminal, check the default port:

```powershell
Invoke-RestMethod http://localhost:3000/health # OK
Invoke-RestMethod http://localhost:3000/       # Hello from Rust
```

Use your configured port if you changed `PORT`. Press Ctrl+C in the server terminal to stop it.

## Development checks

Run from `backend`:

```powershell
cargo fmt --check
cargo check --locked
cargo test --locked
```
