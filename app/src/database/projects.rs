use {
  super::_prelude::*,
  pulldown_cmark::{
    Options,
    Parser,
    html,
  },
};

// ── Public models ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Project {
  pub id :          i64,
  pub title :       String,
  pub slug :        String,
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

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct ProjectDetail {
  pub id :          i64,
  pub title :       String,
  pub slug :        String,
  pub description : String,
  pub status :      String,
  pub repo_url :    Option<String,>,
  pub live_url :    Option<String,>,
  pub featured :    bool,
  pub sort_order :  i64,
  pub created_at :  String,
  pub readme_html : Option<String,>,
  pub screenshots : Vec<String,>,
  pub tags :        Vec<String,>,
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn split_tags(raw : &str,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

// slug is nullable in the schema (ALTER TABLE can't add NOT NULL),
// but all rows are populated by the migration. Default to empty string
// for safety.
#[cfg(feature = "ssr")]
#[derive(FromRow,)]
struct ProjectRow {
  id :          i64,
  title :       String,
  slug :        Option<String,>,
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
#[derive(FromRow,)]
struct TagRow {
  tag : String,
}

#[cfg(feature = "ssr")]
fn row_to_project(r : ProjectRow,) -> Project {
  Project {
    id :          r.id,
    title :       r.title,
    slug :        r.slug.unwrap_or_default(),
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

/// Get a single project with full detail (including README + screenshots) by slug.
/// If readme_html is empty but repo_url exists, fetches from GitHub and caches.
#[server(GetProjectBySlug)]
pub async fn get_project_by_slug(slug : String,) -> Result<Option<ProjectDetail,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct DetailRow {
    id :          i64,
    title :       String,
    slug :        Option<String,>,
    description : String,
    status :      String,
    repo_url :    Option<String,>,
    live_url :    Option<String,>,
    featured :    i64,
    sort_order :  i64,
    created_at :  String,
    readme_html : Option<String,>,
    screenshots : Option<String,>,
    tags :        String,
  }

  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(DetailRow, "sql/projects/get_by_slug.sql", slug)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;

  let Some(r,) = row else {
    return Ok(None,);
  };

  let mut readme_html = r.readme_html.clone();

  // If no cached README but we have a repo URL, fetch from GitHub
  if (readme_html.is_none() || readme_html.as_ref().is_some_and(String::is_empty,))
    && let Some(ref repo_url,) = r.repo_url
    && let Some(fetched,) = fetch_github_readme(repo_url,).await
  {
    // Cache it
    let _ = query_file!("sql/projects/update_readme.sql", r.id, fetched)
      .execute(&pool,)
      .await;
    readme_html = Some(fetched,);
  }

  // Parse screenshots from DB, or extract from README
  let screenshots = r.screenshots.as_ref().map_or_else(
    || {
      readme_html
        .as_ref()
        .map_or_else(Vec::new, |html| extract_images_from_html(html,),)
    },
    |s| split_tags(s,),
  );

  Ok(Some(ProjectDetail {
    id : r.id,
    title : r.title,
    slug : r.slug.unwrap_or_default(),
    description : r.description,
    status : r.status,
    repo_url : r.repo_url,
    live_url : r.live_url,
    featured : r.featured != 0,
    sort_order : r.sort_order,
    created_at : r.created_at,
    readme_html,
    screenshots,
    tags : split_tags(&r.tags,),
  },),)
}

// ── GitHub README fetcher ───────────────────────────────────────────────────

#[cfg(feature = "ssr")]
async fn fetch_github_readme(repo_url : &str,) -> Option<String,> {
  let parts : Vec<&str,> = repo_url
    .trim_end_matches('/',)
    .rsplit('/',)
    .take(2,)
    .collect();
  if parts.len() < 2 {
    return None;
  }
  let (repo, owner,) = (parts[0], parts[1],);

  let raw_url = format!("https://raw.githubusercontent.com/{owner}/{repo}/HEAD/README.md",);

  let client = reqwest::Client::builder()
    .timeout(std::time::Duration::from_secs(10,),)
    .build()
    .ok()?;

  let resp = client.get(&raw_url,).send().await.ok()?;

  if !resp.status().is_success() {
    return None;
  }

  let md = resp.text().await.ok()?;

  let mut opts = Options::empty();
  opts.insert(Options::ENABLE_TABLES,);
  opts.insert(Options::ENABLE_STRIKETHROUGH,);
  opts.insert(Options::ENABLE_TASKLISTS,);
  opts.insert(Options::ENABLE_HEADING_ATTRIBUTES,);

  let parser = Parser::new_ext(&md, opts,);
  let mut out = String::new();
  html::push_html(&mut out, parser,);

  let base = format!("https://raw.githubusercontent.com/{owner}/{repo}/HEAD/",);
  let out = out.replace("src=\"./", &format!("src=\"{base}",),);
  let out = out.replace("src=\"images/", &format!("src=\"{base}images/",),);
  let out = out.replace("src=\"assets/", &format!("src=\"{base}assets/",),);
  let out = out.replace("src=\"docs/", &format!("src=\"{base}docs/",),);

  let sanitized = ammonia::Builder::default()
    .add_tags(["img", "figure", "figcaption", "details", "summary",],)
    .add_tag_attributes("img", [
      "src", "alt", "title", "loading", "decoding", "width", "height",
    ],)
    .add_tag_attributes("a", ["href", "title",],)
    .url_schemes(["http", "https",].into(),)
    .url_relative(ammonia::UrlRelative::PassThrough,)
    .clean(&out,)
    .to_string();

  Some(sanitized,)
}

#[cfg(feature = "ssr")]
fn extract_images_from_html(html : &str,) -> Vec<String,> {
  let mut images = Vec::new();
  let mut rest = html;
  while let Some(start,) = rest.find("src=\"",) {
    let after = &rest[start + 5 ..];
    if let Some(end,) = after.find('"',) {
      let url = &after[.. end];
      if Path::new(url,)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("png",),)
        || Path::new(url,)
          .extension()
          .is_some_and(|ext| ext.eq_ignore_ascii_case("jpg",),)
        || Path::new(url,)
          .extension()
          .is_some_and(|ext| ext.eq_ignore_ascii_case("jpeg",),)
        || Path::new(url,)
          .extension()
          .is_some_and(|ext| ext.eq_ignore_ascii_case("gif",),)
        || Path::new(url,)
          .extension()
          .is_some_and(|ext| ext.eq_ignore_ascii_case("webp",),)
        || Path::new(url,)
          .extension()
          .is_some_and(|ext| ext.eq_ignore_ascii_case("svg",),)
      {
        images.push(url.to_string(),);
      }
      rest = &after[end ..];
    } else {
      break;
    }
  }
  images
}
