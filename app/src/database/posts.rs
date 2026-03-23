pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct PostSummary {
  pub id :           i64,
  pub title :        String,
  pub slug :         String,
  pub excerpt :      Option<String,>,
  pub kind :         String,
  pub featured :     bool,
  pub cover_url :    Option<String,>,
  pub published_at : Option<String,>,
  pub created_at :   String,
  pub tags :         Vec<String,>,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Post {
  pub id :           i64,
  pub title :        String,
  pub slug :         String,
  pub body :         String,
  pub excerpt :      Option<String,>,
  pub kind :         String,
  pub cover_url :    Option<String,>,
  pub published_at : Option<String,>,
  pub created_at :   String,
  pub tags :         Vec<String,>,
}

fn split_tags(raw : &str,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

#[server(ListPosts)]
pub async fn list_posts() -> Result<Vec<PostSummary,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :           i64,
    title :        String,
    slug :         String,
    excerpt :      Option<String,>,
    kind :         String,
    featured :     i64,
    cover_url :    Option<String,>,
    published_at : Option<String,>,
    created_at :   String,
    tags :         String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/posts/list.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| PostSummary {
        id :           r.id,
        title :        r.title,
        slug :         r.slug,
        excerpt :      r.excerpt,
        kind :         r.kind,
        featured :     r.featured != 0,
        cover_url :    r.cover_url,
        published_at : r.published_at,
        created_at :   r.created_at,
        tags :         split_tags(&r.tags,),
      },)
      .collect(),
  )
}

#[server(GetPostBySlug)]
pub async fn get_post_by_slug(slug : String,) -> Result<Option<Post,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :           i64,
    title :        String,
    slug :         String,
    body :         String,
    excerpt :      Option<String,>,
    kind :         String,
    cover_url :    Option<String,>,
    published_at : Option<String,>,
    created_at :   String,
    tags :         String,
  }
  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(Row, "sql/posts/get_by_slug.sql", slug)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(row.map(|r| Post {
    id :           r.id,
    title :        r.title,
    slug :         r.slug,
    body :         r.body,
    excerpt :      r.excerpt,
    kind :         r.kind,
    cover_url :    r.cover_url,
    published_at : r.published_at,
    created_at :   r.created_at,
    tags :         split_tags(&r.tags,),
  },),)
}

/// Full-text search across title, excerpt and body using FTS5.
/// Query words are suffixed with * for prefix/fuzzy matching:
///   "rust lep" → "rust* lep*" matches "Rust", "Leptos" etc.
#[server(SearchPosts)]
pub async fn search_posts(query : String,) -> Result<Vec<PostSummary,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :           i64,
    title :        String,
    slug :         String,
    excerpt :      Option<String,>,
    kind :         String,
    featured :     i64,
    cover_url :    Option<String,>,
    published_at : Option<String,>,
    created_at :   String,
    tags :         String,
  }

  // Build prefix query — each whitespace-separated word gets a wildcard
  // "rust lep" → "rust* lep*" matches "Rust", "Leptos" etc.
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
  let rows = query_file_as!(Row, "sql/posts/search.sql", fts_query)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;

  Ok(
    rows
      .into_iter()
      .map(|r| PostSummary {
        id :           r.id,
        title :        r.title,
        slug :         r.slug,
        excerpt :      r.excerpt,
        kind :         r.kind,
        featured :     r.featured != 0,
        cover_url :    r.cover_url,
        published_at : r.published_at,
        created_at :   r.created_at,
        tags :         split_tags(&r.tags,),
      },)
      .collect(),
  )
}

/// Distinct post kinds for filter buttons.
#[server(ListPostKinds)]
pub async fn list_post_kinds() -> Result<Vec<String,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    kind : String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/posts/list_kinds.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(|r| r.kind,).collect(),)
}

/// All published posts filtered by kind.
#[server(ListPostsByKind)]
pub async fn list_posts_by_kind(kind : String,) -> Result<Vec<PostSummary,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :           i64,
    title :        String,
    slug :         String,
    excerpt :      Option<String,>,
    kind :         String,
    featured :     i64,
    cover_url :    Option<String,>,
    published_at : Option<String,>,
    created_at :   String,
    tags :         String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/posts/list_by_kind.sql", kind)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| PostSummary {
        id :           r.id,
        title :        r.title,
        slug :         r.slug,
        excerpt :      r.excerpt,
        kind :         r.kind,
        featured :     r.featured != 0,
        cover_url :    r.cover_url,
        published_at : r.published_at,
        created_at :   r.created_at,
        tags :         split_tags(&r.tags,),
      },)
      .collect(),
  )
}
