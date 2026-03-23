pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Project {
  pub id :          i64,
  pub title :       String,
  pub description : String,
  /// `"active"` | `"building"` | `"planning"` | `"archived"`
  pub status :      String,
  pub repo_url :    Option<String,>,
  pub live_url :    Option<String,>,
  pub featured :    bool,
  pub sort_order :  i64,
  pub created_at :  String,
  pub tags :        Vec<String,>,
}

fn split_tags(raw : &str,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

// ── Shared row type + mapper ────────────────────────────────────────────────

#[cfg(feature = "ssr")]
#[derive(FromRow,)]
struct ProjectRow {
  id :          i64,
  title :       String,
  description : String,
  status :      String,
  repo_url :    Option<String,>,
  live_url :    Option<String,>,
  featured :    i64,
  sort_order :  i64,
  created_at :  String,
  tags :        String,
}

#[cfg(feature = "ssr")]
fn row_to_project(r : ProjectRow,) -> Project {
  Project {
    id :          r.id,
    title :       r.title,
    description : r.description,
    status :      r.status,
    repo_url :    r.repo_url,
    live_url :    r.live_url,
    featured :    r.featured != 0,
    sort_order :  r.sort_order,
    created_at :  r.created_at,
    tags :        split_tags(&r.tags,),
  }
}

// ── Server functions ────────────────────────────────────────────────────────

/// All published projects ordered by `sort_order`, then newest first.
#[server(ListProjects)]
pub async fn list_projects() -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/list.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

/// Featured projects for the Home page (max 4).
#[server(GetFeaturedProjects)]
pub async fn get_featured_projects() -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/get_featured.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

/// Single published project by numeric id.
#[server(GetProjectById)]
pub async fn get_project_by_id(id : i64,) -> Result<Option<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(ProjectRow, "sql/projects/get_by_id.sql", id)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(row.map(row_to_project,),)
}

/// Distinct tags across all published projects.
#[server(ListProjectTags)]
pub async fn list_project_tags() -> Result<Vec<String,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct TagRow {
    tag : String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(TagRow, "sql/projects/list_tags.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(|r| r.tag,).collect(),)
}

/// Projects filtered by status.
#[server(ListProjectsByStatus)]
pub async fn list_projects_by_status(status : String,) -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/list_by_status.sql", status)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

/// Projects filtered by tag.
#[server(ListProjectsByTag)]
pub async fn list_projects_by_tag(tag : String,) -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/list_by_tag.sql", tag)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

/// Full-text search across project title and description using FTS5.
#[server(SearchProjects)]
pub async fn search_projects(query : String,) -> Result<Vec<Project,>, ServerFnError,> {
  let fts_query = query
    .split_whitespace()
    .filter(|w| !w.is_empty(),)
    .map(|w| format!("{w}*",),)
    .collect::<Vec<_,>>()
    .join(" ",);

  if fts_query.is_empty() {
    return Ok(vec![],);
  }

  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/search.sql", fts_query)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}
