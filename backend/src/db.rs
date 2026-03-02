use sqlx::{
  SqlitePool,
  migrate,
  query,
  sqlite::SqlitePoolOptions,
};

/// Create the connection pool and apply pending migrations.
///
/// Migrations are embedded at compile time from `database/migrations/`
/// and applied automatically on startup — no manual `sqlx migrate run`
/// required in production.
pub async fn init(database_url : &str,) -> anyhow::Result<SqlitePool,> {
  let pool = SqlitePoolOptions::new()
    .max_connections(6,)
    .connect(database_url,)
    .await?;

  query("PRAGMA journal_mode=WAL;",).execute(&pool,).await?;
  migrate!("../database/migrations").run(&pool,).await?;

  Ok(pool,)
}
