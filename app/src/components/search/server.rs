use super::_prelude::*;

#[server(SitewideSearch)]
pub async fn sitewide_search(query: String) -> Result<Vec<Item>, ServerFnError> {
  let fts_query = query
    .split_whitespace()
    .filter(|w| !w.is_empty())
    .map(|w| format!("{w}*",))
    .collect::<Vec<_>>()
    .join(" ");

  if fts_query.is_empty() {
    return Ok(vec![]);
  }

  let mut results = Vec::new();

  if let Ok(projects) = search_projects(query.clone()).await {
    results.extend(projects.into_iter().take(5).map(|p| Item {
      url: format!("/dev/{}", p.slug),
      title: p.title,
      subtitle: p.description,
      kind: Kind::Project,
    }));
  }

  if let Ok(media) = search_media(query.clone()).await {
    results.extend(media.into_iter().take(5).map(|m| Item {
      url: format!("/art/{}", m.slug),
      title: m.title,
      subtitle: m.caption.unwrap_or_default(),
      kind: Kind::Art,
    }));
  }

  if let Ok(posts) = search_posts(query).await {
    results.extend(posts.into_iter().take(5).map(|p| Item {
      url: format!("/log/{}", p.slug),
      title: p.title,
      subtitle: p.excerpt.unwrap_or_default(),
      kind: Kind::Log,
    }));
  }

  Ok(results)
}

#[server(GetContextTags)]
pub async fn get_context_tags(
  page: String,
) -> Result<Option<(String, Vec<String>)>, ServerFnError> {
  use crate::database::posts::list_post_kinds;
  match page.as_str() {
    | "/dev" => Ok(
      list_project_tags()
        .await
        .ok()
        .map(|t| ("Technologies".into(), t)),
    ),
    | "/art" => Ok(list_media_tags().await.ok().map(|t| ("Tags".into(), t))),
    | "/log" => Ok(list_post_kinds().await.ok().map(|k| ("Kinds".into(), k))),
    | _ => Ok(None),
  }
}
