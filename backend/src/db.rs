// Database pool initialisation.
//
// Called once at startup from main(). The returned pool is cloned into
// AppState and provided to all server functions via Leptos context.
//
// MIGRATIONS
//   sqlx::migrate! embeds every file in migrations/ at compile time (sorted
//   by filename). On startup it applies any pending migrations automatically —
//   no manual `sqlx migrate run` step required in production.
//
// PATH
//   The path passed to sqlx::migrate! is relative to CARGO_MANIFEST_DIR of
//   the `backend` crate, which is backend/. So "../migrations" resolves to
//   the workspace-root migrations/ directory.

use sqlx::{
  SqlitePool,
  migrate,
  query,
  sqlite::SqlitePoolOptions,
};

/// Create the connection pool and apply pending migrations.
///
/// `database_url` should be `"sqlite:./portfolio.db"` or
/// `"sqlite::memory:"` for tests.
pub async fn init(database_url : &str,) -> anyhow::Result<SqlitePool,> {
  let pool = SqlitePoolOptions::new()
    // One writer + up to 5 readers. SQLite WAL mode handles this well.
    .max_connections(6,)
    .connect(database_url,)
    .await?;

  // WAL gives better concurrent read performance at portfolio scale.
  query("PRAGMA journal_mode=WAL;",).execute(&pool,).await?;

  // Apply any pending migrations embedded at compile time.
  migrate!("../migrations").run(&pool,).await?;

  Ok(pool,)
}
