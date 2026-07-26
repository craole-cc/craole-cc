# Content database sync

## Summary

This phase adds `content sync-db`, a one-command path for hydrating the local SQLite runtime cache
from Git-tracked content.

## Command

```sh
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db
```

If the database URL is omitted, `sync-db` uses `DATABASE_URL`, then falls back to
`sqlite://database/data/portfolio.db`.

## Behavior

`sync-db` performs the existing manual workflow inside the CLI:

1. Validate content under `assets/`.
2. Create the parent database directory if needed.
3. Open the SQLite database.
4. Apply `database/migrations/*.sql` in sorted order.
5. Generate seed SQL from Git-tracked content.
6. Apply the seed SQL.
7. Print final counts for projects, posts, and media.

Expected success output:

```text
content database synced: projects=1 posts=1 media=0
```

Invalid content blocks the sync before creating/opening the database.

## Implementation note

`content` uses `rusqlite 0.32` because it is compatible with the `libsqlite3-sys` version already
required by `sqlx 0.8.6` in the workspace. Newer `rusqlite` versions select a newer
`libsqlite3-sys` and conflict because Cargo allows only one crate with `links = "sqlite3"`.

## Verification

Focused TDD loop:

```text
cargo test -p content --test sync_db
```

Full local quality gate:

```text
nix develop --command bash scripts/ci.sh
```
