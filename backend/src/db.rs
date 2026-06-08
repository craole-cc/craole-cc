use {
  sqlx::{
    SqlitePool,
    migrate,
    query,
    sqlite::{
      SqliteConnectOptions,
      SqlitePoolOptions,
    },
  },
  std::{
    path::Path,
    str::FromStr,
  },
};

/// Create the connection pool and apply pending migrations.
///
/// Migrations are embedded at compile time from `database/migrations/`
/// and applied automatically on startup — no manual `sqlx migrate run`
/// required in production.
pub async fn init(database_url : &str,) -> anyhow::Result<SqlitePool,> {
  create_parent_dir(database_url,)?;

  let options = SqliteConnectOptions::from_str(database_url,)?.create_if_missing(true,);
  let pool = SqlitePoolOptions::new()
    .max_connections(6,)
    .connect_with(options,)
    .await?;

  query("PRAGMA journal_mode=WAL;",).execute(&pool,).await?;
  migrate!("../database/migrations").run(&pool,).await?;

  Ok(pool,)
}

fn create_parent_dir(database_url : &str,) -> anyhow::Result<(),> {
  let Some(path,) = database_url
    .strip_prefix("sqlite://",)
    .or_else(|| database_url.strip_prefix("sqlite:",),)
  else {
    return Ok((),);
  };

  let path = Path::new(path,);
  if let Some(parent,) = path
    .parent()
    .filter(|parent| !parent.as_os_str().is_empty(),)
  {
    std::fs::create_dir_all(parent,)?;
  }

  Ok((),)
}
