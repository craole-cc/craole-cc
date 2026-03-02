use super::_prelude::*;

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

fn split_tags(raw : String,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

/// All published projects ordered by `sort_order`, then newest first.
#[server(ListProjects)]
pub async fn list_projects() -> Result<Vec<Project,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :          i64,
    title :       String,
    description : String,
    status :      String,
    repo_url :    Option<String,>,
    live_url :    Option<String,>,
    featured :    i64, // SQLite INTEGER 0/1 — no native bool
    sort_order :  i64,
    created_at :  String,
    tags :        String, // COALESCE guarantees non-null TEXT
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/projects/list.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Project {
        id :          r.id,
        title :       r.title,
        description : r.description,
        status :      r.status,
        repo_url :    r.repo_url,
        live_url :    r.live_url,
        featured :    r.featured != 0,
        sort_order :  r.sort_order,
        created_at :  r.created_at,
        tags :        split_tags(r.tags,),
      },)
      .collect(),
  )
}

/// Featured projects for the Home page (max 4).
#[server(GetFeaturedProjects)]
pub async fn get_featured_projects() -> Result<Vec<Project,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
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
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/projects/get_featured.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Project {
        id :          r.id,
        title :       r.title,
        description : r.description,
        status :      r.status,
        repo_url :    r.repo_url,
        live_url :    r.live_url,
        featured :    r.featured != 0,
        sort_order :  r.sort_order,
        created_at :  r.created_at,
        tags :        split_tags(r.tags,),
      },)
      .collect(),
  )
}

/// Single published project by numeric id.
#[server(GetProjectById)]
pub async fn get_project_by_id(id : i64,) -> Result<Option<Project,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
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
  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(Row, "sql/projects/get_by_id.sql", id)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(row.map(|r| Project {
    id :          r.id,
    title :       r.title,
    description : r.description,
    status :      r.status,
    repo_url :    r.repo_url,
    live_url :    r.live_url,
    featured :    r.featured != 0,
    sort_order :  r.sort_order,
    created_at :  r.created_at,
    tags :        split_tags(r.tags,),
  },),)
}
