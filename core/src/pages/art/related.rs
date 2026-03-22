use crate::prelude::*;

#[component]
pub fn Related(slug : String,) -> impl IntoView {
  let related = Resource::new(
    move || slug.clone(),
    |s| async move { list_related_media(s,).await },
  );

  view! {
    <Suspense fallback=|| ()>
      {move || {
        related
          .get()
          .map(|res: Result<Vec<Media>, ServerFnError>| {
            res
              .ok()
              .and_then(|items: Vec<Media>| {
                if items.is_empty() {
                  return None;
                }
                Some(
                  view! {
                    <aside class="art-related readable">
                      <h2 class="art-related__heading">"Related"</h2>
                      <ul class="art-related__grid" role="list">
                        {items
                          .into_iter()
                          .map(|item| {
                            view! {
                              <li class="art-related__item">
                                <a href=format!("/art/{}", item.slug) class="art-related__link">
                                  <figure class="art-related__figure">
                                    <img
                                      src=item.file_path
                                      alt=item.alt_text
                                      loading="lazy"
                                      decoding="async"
                                      class="art-related__img"
                                    />
                                    <figcaption class="art-related__title">{item.title}</figcaption>
                                  </figure>
                                </a>
                              </li>
                            }
                          })
                          .collect_view()}
                      </ul>
                    </aside>
                  },
                )
              })
          })
      }}
    </Suspense>
  }
}
