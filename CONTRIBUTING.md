# Contributing to craole.cc

Thanks for your interest! This document covers how to get the project running locally.

---

## Prerequisites

| Tool | Purpose | Install |
|------|---------|---------|
| Rust (nightly) | Compiler — version pinned in `rust-toolchain.toml` | [rustup.rs](https://rustup.rs) |
| `cargo-leptos` | Full-stack dev server & build tool | `cargo install cargo-leptos` |
| `sqlx-cli` | Database migrations & compile-time query checks | `cargo install sqlx-cli --no-default-features --features sqlite` |
| Node.js / npm | Optional — only needed if you add JS dependencies | [nodejs.org](https://nodejs.org) |

> If you're on NixOS, a `flake.nix` is included — `nix develop` will drop you into a shell with everything available.

---

## Setup

### 1. Clone the repo

```sh
git clone https://github.com/craole-cc/craole-cc.git
cd craole-cc
```

### 2. Configure environment

Copy the example env file and adjust if needed:

```sh
cp .env.example .env
```

The default is fine for local development:

```env
DATABASE_URL=sqlite:./database/data/portfolio.db
```

### 3. Initialize the database

```sh
chmod +x scripts/init-db.sh
./scripts/init-db.sh
```

This creates the SQLite database file and runs all migrations. SQLx verifies
queries against a live database at compile time, so this step must happen
before you can build.

To wipe and start fresh:

```sh
./scripts/init-db.sh --reset   # also accepts -f or --force
```

### 4. Start the dev server

```sh
cargo leptos watch
```

The app will be available at `http://127.0.0.1:3000`. The server hot-reloads
on changes to Rust, SCSS, and assets.

---

## Project Structure

```
.
├── backend/        # Axum server entry point
├── core/           # Shared library — pages, components, DB queries, theme
│   └── sql/        # SQL query files (used by sqlx query_file_as! macros)
├── frontend/       # WASM frontend entry point
├── database/
│   ├── data/       # SQLite database file (gitignored)
│   └── migrations/ # SQL migration files
├── style/          # SCSS source files
├── content/        # Markdown content (posts, CV)
├── scripts/        # Dev tooling scripts
└── public/         # Static assets
```

---

## Notes

- The backend automatically applies pending migrations on startup via
  `sqlx::migrate!` — you don't need to run `sqlx migrate run` manually
  after the initial setup.
- `SQLX_OFFLINE=true` can be set to skip live DB checks during compilation,
  but requires a pre-generated `.sqlx/` cache (`cargo sqlx prepare --workspace`).
  Leave it unset for local dev.
- The `database/data/` directory is gitignored. Never commit the `.db` file.
