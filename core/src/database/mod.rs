// Database access layer.
//
// ARCHITECTURE
//   Each sub-module owns one content domain (projects, media, posts).
//   It contains:
//     1. Public model struct   — Serialize + Deserialize, compiled for both client and server
//        (needed for server function signatures on WASM).
//     2. Internal row struct   — sqlx::FromRow, defined *inside* #[server] function bodies so it
//        never reaches the WASM artifact.
//     3. Server functions      — #[server] generates the client stub; the body (with sqlx calls)
//        only runs on the server.
//
// SERVER FUNCTION PATTERN
//   Every server function body:
//     1. Extracts SqlitePool from Leptos context (injected in backend/src/main.rs via
//        leptos_routes_with_context)
//     2. Runs query_file_as! against a .sql file in core/sql/
//     3. Maps the row type to the public model
//     4. Returns Result<T, ServerFnError>
//
// SQL FILE PATHS
//   query_file_as! paths are relative to CARGO_MANIFEST_DIR of the *core*
//   crate, so "sql/projects/list.sql" resolves to core/sql/projects/list.sql.
//
// ADMIN NOTE
//   Write operations (INSERT / UPDATE / DELETE) are intentionally absent here.
//   They will live in a separate admin module, protected by Axum middleware
//   defined in backend/src/main.rs. See backend/src/main.rs for the plan.

pub mod media;
pub mod posts;
pub mod projects;

pub mod _prelude {
  pub use crate::prelude::*;
  #[cfg(feature = "ssr")]
  pub use sqlx::{
    FromRow,
    SqlitePool,
    query_file_as,
  };
}

pub mod prelude {
  pub use super::{
    media::{
      Media,
      get_media_by_id,
      list_media,
    },
    posts::{
      GetPostBySlug,
      Post,
      PostSummary,
      get_post_by_slug,
      list_posts,
    },
    projects::{
      Project,
      get_featured_projects,
      get_project_by_id,
      list_projects,
    },
  };
}
