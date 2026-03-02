use super::_prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Media {
  pub id :         i64,
  pub title :      String,
  pub caption :    Option<String,>,
  pub media_type : String,
  pub file_path :  String,
  pub alt_text :   String,
  pub width :      Option<i64,>,
  pub height :     Option<i64,>,
  pub sort_order : i64,
  pub taken_at :   Option<String,>,
  pub created_at : String,
}

/// All published media ordered for gallery display.
#[server(ListMedia)]
pub async fn list_media() -> Result<Vec<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
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
        caption :    r.caption,
        media_type : r.media_type,
        file_path :  r.file_path,
        alt_text :   r.alt_text,
        width :      r.width,
        height :     r.height,
        sort_order : r.sort_order,
        taken_at :   r.taken_at,
        created_at : r.created_at,
      },)
      .collect(),
  )
}

/// Single published media item by id.
#[server(GetMediaById)]
pub async fn get_media_by_id(id : i64,) -> Result<Option<Media,>, ServerFnError,> {
  #[derive(FromRow,)]
  struct Row {
    id :         i64,
    title :      String,
    caption :    Option<String,>,
    media_type : String,
    file_path :  String,
    alt_text :   String,
    width :      Option<i64,>,
    height :     Option<i64,>,
    sort_order : i64,
    taken_at :   Option<String,>,
    created_at : String,
  }
  let pool = expect_context::<SqlitePool,>();
  let row = query_file_as!(Row, "sql/media/get_by_id.sql", id)
    .fetch_optional(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(row.map(|r| Media {
    id :         r.id,
    title :      r.title,
    caption :    r.caption,
    media_type : r.media_type,
    file_path :  r.file_path,
    alt_text :   r.alt_text,
    width :      r.width,
    height :     r.height,
    sort_order : r.sort_order,
    taken_at :   r.taken_at,
    created_at : r.created_at,
  },),)
}
