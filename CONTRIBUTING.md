# Contributing to [Craole.CC](https://craole.cc)

Thanks for your interest! This document covers how to get the project running locally, how the
content pipeline works, and what to run before opening a pull request.

---

## Project Goals

This repository is a Rust portfolio site with a deliberate dual-track architecture:

1. **Robust application path** — Leptos + Axum + SQLite, server-rendered with WASM hydration.
2. **Resilient content path** — Git-tracked Markdown/TOML content that can be validated and exported
   to SQLite for local development, CI, and future static snapshots.

When contributing, keep both paths healthy. A change should improve the full app without making the
content source of truth harder to validate, seed, or eventually pre-render.

---

## Prerequisites

| Tool | Purpose | Install |
| --- | --- | --- |
| Rust nightly | Compiler — pinned in `rust-toolchain.toml` | [rustup.rs](https://rustup.rs) |
| `cargo-leptos` | Full-stack dev server and build tool | `cargo install cargo-leptos` |
| `sqlx-cli` | Database migrations and compile-time query checks | `cargo install sqlx-cli --no-default-features --features sqlite` |
| `sqlite3` | Local SQLite inspection and seed smoke tests | Package manager, usually `sqlite` or `sqlite3` |
| Node.js / npm | Optional — only needed if you add JS dependencies | [nodejs.org](https://nodejs.org) |

> Nix users: `flake.nix` is included. `nix develop` should provide Rust, `cargo-leptos`, `sqlx`,
> `sqlite3`, and the supporting repository tools.

---

## Fresh Checkout Setup

### 1. Clone the Repo

```sh
git clone https://github.com/craole-cc/craole-cc.git
cd craole-cc
```

### 2. Enter the Dev Environment

Preferred:

```sh
nix develop
```

Without Nix, make sure these commands work before continuing:

```sh
rustc --version
cargo --version
cargo leptos --version
sqlx --version
sqlite3 --version
```

### 3. Configure Environment

```sh
cp .env.example .env
```

The default local database URL is:

```env
DATABASE_URL=sqlite://./database/data/portfolio.db
```

`SQLX_OFFLINE=true` may be useful for builds that rely on committed `.sqlx/` metadata, but leave it
unset or set it to `false` when refreshing query metadata against a live local database.

### 4. Initialize the Database

```sh
./scripts/init-db.rs
```

To wipe and start fresh:

```sh
./scripts/init-db.rs --reset
```

The script creates `database/data/portfolio.db` and applies migrations from `database/migrations/`.
The database file is intentionally ignored from git.

### 5. Start the Dev Server

```sh
leptoswatch
```

The app will be available at `http://127.0.0.1:3000`. The server hot-reloads on changes to Rust,
Sass, and assets. Use `leptoswatch 3005` to run on another port; the reload port is set to the next
port (`3006` in that example). If either port is already occupied, the shortcut prints the listener
with `lsof` and asks whether to kill it before starting `cargo leptos watch`.

---

## Repository Structure

```text
.
├── app/                  # Shared Leptos UI, pages, components, theme, SQL query wrappers
│   ├── sql/              # SQL query files used by SQLx query_file_as! macros
│   └── src/
├── backend/              # Axum server entry point and database bootstrap
├── frontend/             # WASM hydration entry point
├── content/
│   ├── assets/           # Private authored Markdown/TOML source of truth
│   │   ├── posts/        # Blog, note, and profile Markdown
│   │   ├── projects/     # Project TOML records
│   │   └── media/        # Standalone/project media metadata
│   ├── src/              # Content CLI implementation
│   └── SCHEMA.md         # Complete content format and validation rules
├── database/
│   ├── data/             # Local SQLite database files; gitignored
│   └── migrations/       # Durable schema and baseline SQL migrations
├── assets/                # Publicly served images, project media, audio, fonts, and icons
├── docs/                  # Owner runbooks and focused documentation
├── scripts/               # Dev and CI helper scripts
├── style/                 # SCSS and Tailwind input
└── tests/                # Workspace-level tests and fixtures
```

---

## Content Workflow

Content should start in `content/`, not as ad-hoc SQL edits.

For the copyable owner workflow—adding a blog, project, standalone image, Unsplash image, or project
media and publishing it through production—see [Updating Craole.CC](./docs/UPDATING_SITE.md).

### Create a Draft Template

Use `content new` to create the right filepath with starter frontmatter/fields:

```sh
cargo run -p content -- new project my-project
cargo run -p content -- new post project-build-log
cargo run -p content -- new media portfolio-screenshot
```

Generated project and post templates are valid unpublished drafts. Media templates still require you
to add the referenced asset under ``assets/` before validation will pass.

Invalid slugs are rejected, and existing content files will not be overwritten.

### Validate Content

```sh
cargo run -p content -- validate .
```

Expected success output:

```text
content validation passed
```

### Export Content into SQLite

After migrations have created the schema, export the Git-tracked content into the local database:

```sh
cargo run -p content -- export-sql . | sqlite3 database/data/portfolio.db
```

This is the same smoke-test path used by `scripts/ci.sh`. It currently checks that at least one
project and one post are seeded.

### Sync Content into SQLite

Use `sync-db` when you want `content` to apply migrations and seed the local database in one step:

```sh
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db
```

If the database URL argument is omitted, `sync-db` uses `DATABASE_URL` and then falls back to
`sqlite://database/data/portfolio.db`.

Successful output includes seeded table counts:

```text
content database synced: projects=1 posts=1 media=0
```

### Export Static JSON

Use `export-json` to generate static-friendly content data for a fallback site or future pre-renderer:

```sh
cargo run -p content -- export-json . dist/data
```

This writes:

```text
dist/data/projects.json
dist/data/posts.json
dist/data/media.json
dist/data/manifest.json
```

The command validates content before writing files and refuses to export invalid content.

### Export a Static Fallback Site

Use `export-static` to generate a minimal HTML fallback site plus the same JSON data files:

```sh
cargo run -p content -- export-static . dist
```

This writes:

```text
dist/index.html
dist/dev/index.html
dist/dev/<project-slug>/index.html
dist/log/index.html
dist/log/<post-slug>/index.html
dist/art/index.html
dist/404.html
dist/sitemap.xml
dist/data/*.json
```

The generated HTML is intentionally simple: it is a deployment safety net and preview artifact, not a
replacement for the full Leptos/Axum SSR site.

### Add a Project

1. Create a draft template:

   ```sh
   cargo run -p content -- new project <slug>
   ```

2. Edit `content/assets/projects/<slug>.toml`.
3. Use a lowercase hyphenated `slug`.
4. Include at minimum `title`, `slug`, `status`, and `description`.
5. If `featured = true`, also set `published = true`.
6. Run `cargo run -p content -- validate .`.
7. Run `bash scripts/ci.sh` before committing.

See [content/SCHEMA.md](./content/SCHEMA.md) for complete field rules and examples.

### Add a Post

1. Create a draft template:

   ```sh
   cargo run -p content -- new post <slug>
   ```

2. Edit `content/assets/posts/<slug>.md`.
3. Add frontmatter with `title`, `slug`, and `kind`.
4. Use `published_at: "YYYY-MM-DD"` when publishing dated content.
5. Published posts must have a non-empty body.
6. Run content validation and CI before committing.

---

## Database and SQLx Workflow

The backend automatically applies pending migrations on startup via `sqlx::migrate!`, but local
compile-time query checks still need either a live database or committed SQLx metadata.

### Live Database Path

```sh
export DATABASE_URL=sqlite://./database/data/portfolio.db
./scripts/init-db.rs
cargo check --workspace
```

### Refresh SQLx Metadata

Run this after changing SQL queries, migrations, or query result shapes:

```sh
export DATABASE_URL=sqlite://./database/data/portfolio.db
./scripts/init-db.rs --reset
cargo sqlx prepare --workspace --database-url "$DATABASE_URL"
```

Commit the resulting `.sqlx/*.json` changes when they correspond to intentional query/schema
changes.

### Offline Build Path

```sh
SQLX_OFFLINE=true cargo check --workspace
```

If this fails with missing query metadata, refresh SQLx metadata with the live database path above.

---

## Quality Gate Before a PR

Preferred command:

```sh
nix develop --command bash scripts/ci.sh
```

If you are already inside the dev shell:

```sh
bash scripts/ci.sh
```

The gate performs:

1. Toolchain discovery for `cargo`, `rustc`, `sqlx`, and `sqlite3`.
2. SQLite database creation and migrations.
3. `content validate`.
4. `content export-sql` piped into SQLite as a seed smoke test.
5. `cargo sqlx prepare --workspace --check`.
6. `cargo check --workspace`.
7. `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
8. Workspace tests with `cargo nextest` when available, otherwise `cargo test`.

Formatting is intentionally not enforced by default while the current tree is not fully rustfmt-clean.
To opt into formatting checks locally:

```sh
STRICT_FORMAT=1 bash scripts/ci.sh
```

---

## Common Troubleshooting

### `error: required command not found: cargo`

You are outside the Rust/Nix development environment. Enter it with:

```sh
nix develop
```

Or install Rust with `rustup` and ensure `cargo` is on `PATH`.

### `sqlx` Cannot Connect to the Database

Check `DATABASE_URL` and make sure the database directory exists:

```sh
export DATABASE_URL=sqlite://./database/data/portfolio.db
mkdir -p database/data
./scripts/init-db.rs
```

### `sqlx` Offline Metadata Is Stale

Refresh it against a live database:

```sh
export DATABASE_URL=sqlite://./database/data/portfolio.db
./scripts/init-db.rs --reset
cargo sqlx prepare --workspace --database-url "$DATABASE_URL"
```

### Content Seed Creates Empty Tables

Run validation first and confirm that at least one published project and one published post exist:

```sh
cargo run -p content -- validate .
ls content/assets/projects content/assets/posts assets/media assets/audio
```

### Static Assets Referenced by Content Are Missing

- Screenshot paths use the flat image namespace, so `/media/images/demo_home.webp` maps to
  `assets/media/images/demo_home.webp`.

---

## Commit Guidance

- Keep migrations, SQL query files, `.sqlx/` metadata, and content changes in sync.
- Prefer conventional commit messages, e.g. `feat: add project content seed`.
- Do not commit local database files from `database/data/`.
- Include the relevant verification command and result in the PR description.
