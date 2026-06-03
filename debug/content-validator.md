# Content validator implementation notes

## What changed

- Added a new workspace crate: `contentctl`.
- Added `contentctl validate [repo-root]` for local-first content checks.
- Added `contentctl export-sql [repo-root]` to generate a SQLite seed script from content files.
- Added validator coverage for:
  - `content/projects/*.toml`
  - `content/posts/*.md`
  - `content/media/*.toml`
- Wired the validator into `scripts/ci.sh` before SQLx metadata checks.
- Documented the source content schema in `content/SCHEMA.md`.

## Why this shape

The CMS/content pipeline can stay Git-first for now:

```text
content files -> contentctl validate -> SQLite sync / SSR -> static snapshot later
```

That keeps the portfolio deployable without requiring a production admin dashboard yet.

## Validator guarantees

`contentctl` catches the failures that would be annoying to debug after deploy:

- duplicate slugs inside a content type
- invalid project/media status-like fields
- missing required fields
- invalid lowercase-hyphen slug format
- invalid dates for fields that are expected to use `YYYY-MM-DD`
- broken public asset references
- featured-but-unpublished content
- published media without alt text

## Commands

```bash
cargo test -p contentctl --test validation
cargo test -p contentctl --test export_sql
cargo run -p contentctl -- validate .
cargo run -p contentctl -- export-sql . | sqlite3 database/data/portfolio.db
nix develop --command bash scripts/ci.sh
```
