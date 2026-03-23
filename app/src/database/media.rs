use super::_prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Media {
  pub id :         i64,
  pub title :      String,
  pub slug :       String,
  pub caption :    Option<String,>,
  pub media_type : String,
  pub file_path :  String,
  pub alt_text :   String,
  pub width :      Option<i64,>,
  pub height :     Option<i64,>,
  pub sort_order : i64,
  pub taken_at :   Option<String,>,
  pub created_at : String,
  pub tags :       Vec<String,>,
}

fn split_tags(raw : &str,) -> Vec<String,> {
  if raw.is_empty() {
    vec![]
  } else {
    raw.split(',',).map(str::to_string,).collect()
  }
}

#[server(ListMedia)]
pub async fn list_media() -> Result<Vec<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    slug :       String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
    tags :       String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/media/list.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Media {
        id :         r.id,
        title :      r.title,
        slug :       r.slug,
        caption :    r.caption,
        media_type : r.media_type,
        file_path :  r.file_path,
        alt_text :   r.alt_text,
        width :      r.width,
        height :     r.height,
        sort_order : r.sort_order,
        taken_at :   r.taken_at,
        created_at : r.created_at,
        tags :       split_tags(&r.tags,),
      },)
      .collect(),
  )
}

#[server(GetMediaBySlug)]
pub async fn get_media_by_slug(slug : String,) -> Result<Option<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    slug :       String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
    tags :       String,
  }
  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(Row, "sql/media/get_by_id.sql", slug)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(row.map(|r| Media {
    id :         r.id,
    title :      r.title,
    slug :       r.slug,
    caption :    r.caption,
    media_type : r.media_type,
    file_path :  r.file_path,
    alt_text :   r.alt_text,
    width :      r.width,
    height :     r.height,
    sort_order : r.sort_order,
    taken_at :   r.taken_at,
    created_at : r.created_at,
    tags :       split_tags(&r.tags,),
  },),)
}

#[server(SearchMedia)]
pub async fn search_media(query : String,) -> Result<Vec<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    slug :       String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
    tags :       String,
  }
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
  let rows = query_file_as!(Row, "sql/media/search.sql", fts_query)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Media {
        id :         r.id,
        title :      r.title,
        slug :       r.slug,
        caption :    r.caption,
        media_type : r.media_type,
        file_path :  r.file_path,
        alt_text :   r.alt_text,
        width :      r.width,
        height :     r.height,
        sort_order : r.sort_order,
        taken_at :   r.taken_at,
        created_at : r.created_at,
        tags :       split_tags(&r.tags,),
      },)
      .collect(),
  )
}

#[server(ListMediaByTag)]
pub async fn list_media_by_tag(tag : String,) -> Result<Vec<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    slug :       String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
    tags :       String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/media/list_by_tag.sql", tag)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Media {
        id :         r.id,
        title :      r.title,
        slug :       r.slug,
        caption :    r.caption,
        media_type : r.media_type,
        file_path :  r.file_path,
        alt_text :   r.alt_text,
        width :      r.width,
        height :     r.height,
        sort_order : r.sort_order,
        taken_at :   r.taken_at,
        created_at : r.created_at,
        tags :       split_tags(&r.tags,),
      },)
      .collect(),
  )
}

#[server(ListMediaTags)]
pub async fn list_media_tags() -> Result<Vec<String,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    tag : String,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/media/list_tags.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(|r| r.tag,).collect(),)
}

#[server(ListRelatedMedia)]
pub async fn list_related_media(slug : String,) -> Result<Vec<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :               i64,
    title :            String,
    slug :             String,
    caption :          Option<String,>,
    media_type :       String,
    file_path :        String,
    alt_text :         String,
    width :            Option<i64,>,
    height :           Option<i64,>,
    sort_order :       i64,
    taken_at :         Option<String,>,
    created_at :       String,
    tags :             String,
    shared_tag_count : i64,
    same_shoot :       i64,
  }
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(Row, "sql/media/list_related.sql", slug)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(
    rows
      .into_iter()
      .map(|r| Media {
        id :         r.id,
        title :      r.title,
        slug :       r.slug,
        caption :    r.caption,
        media_type : r.media_type,
        file_path :  r.file_path,
        alt_text :   r.alt_text,
        width :      r.width,
        height :     r.height,
        sort_order : r.sort_order,
        taken_at :   r.taken_at,
        created_at : r.created_at,
        tags :       split_tags(&r.tags,),
      },)
      .collect(),
  )
}
