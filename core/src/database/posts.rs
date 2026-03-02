use super::_prelude::*;

/// Lightweight summary used in the Log index list.
#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct PostSummary {
  pub id :           i64,
  pub title :        String,
  pub slug :         String,
  pub excerpt :      Option<String,>,
  pub kind :         String,
  pub published_at : Option<String,>,
  pub created_at :   String,
  pub tags :         Vec<String,>,
}

/// Full post including Markdown body.
#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Post {
  pub id :           i64,
  pub title :        String,
  pub slug :         String,
  pub body :         String,
  pub excerpt :      Option<String,>,
  pub kind :         String,
  pub published_at : Option<String,>,
  pub created_at :   String,
  pub tags :         Vec<String,>,
}

fn split_tags(raw : String,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

/// Summaries of all published posts, newest first.
#[server(ListPosts)]
pub async fn list_posts() -> Result<Vec<PostSummary,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :           i64,
    title :        String,
    slug :         String,
    excerpt :      Option<String,>,
    kind :         String,
    published_at : Option<String,>,
    created_at :   String,
    tags :         String, // COALESCE guarantees non-null TEXT
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
        published_at : r.published_at,
        created_at :   r.created_at,
        tags :         split_tags(r.tags,),
      },)
      .collect(),
  )
}

/// Full post by URL slug.
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
    published_at : r.published_at,
    created_at :   r.created_at,
    tags :         split_tags(r.tags,),
  },),)
}
