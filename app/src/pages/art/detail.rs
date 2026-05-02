use {super::related::Related, crate::prelude::*};

#[component]
pub fn Detail() -> impl IntoView {
  let params = use_params_map();
  let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

  let media = Resource::new(slug, |s| async move {
    if s.is_empty() {
      return Ok(None);
    }
    get_media_by_slug(s).await
  });

  view! {
    <Suspense fallback=move || {
      view! { <p class="art-loading">"Loading…"</p> }.into_any()
    }>
      {move || {
        media
          .get()
          .map(|res: Result<Option<Media>, ServerFnError>| match res {
            Ok(Some(item)) => {
              let slug_for_related = item.slug.clone();
              view! {
                <article class="art-detail">
                  // -- Full-bleed photo ──────────────────────────────────────────
                  <figure class="art-detail__figure">
                    <img
                      class="art-detail__img"
                      src=item.file_path.clone()
                      alt=item.alt_text.clone()
                      loading="eager"
                      decoding="async"
                    />
                    // Metadata overlay — visible on hover via CSS
                    <figcaption class="art-detail__overlay">
                      <nav class="art-detail__back">
                        <a href="/art">"← Art"</a>
                      </nav>
                      <div class="art-detail__meta">
                        <h1 class="art-detail__title">{item.title.clone()}</h1>
                        {item.caption.map(|c| view! { <p class="art-detail__caption">{c}</p> })}
                        {item.taken_at.map(|d| view! { <time class="art-detail__date">{d}</time> })}
                        {(!item.tags.is_empty())
                          .then(|| {
                            view! {
                              <ul class="art-detail__tags" role="list">
                                {item
                                  .tags
                                  .iter()
                                  .map(|t: &String| {
                                    view! { <li class="art-detail__tag">{t.clone()}</li> }
                                  })
                                  .collect_view()}
                              </ul>
                            }
                          })}
                      </div>
                    </figcaption>
                  </figure>

                  // -- Related photos ────────────────────────────────────────────
                  <Related slug=slug_for_related />
                </article>
              }
                .into_any()
            }
            Ok(None) => {

              view! {
                <div class="art-empty readable">
                  <h2>"Photo not found."</h2>
                  <a href="/art">"← Back to Art"</a>
                </div>
              }
                .into_any()
            }
            Err(e) => {

              view! {
                <div class="art-empty readable">
                  <h2>"Something went wrong."</h2>
                  <p>{e.to_string()}</p>
                </div>
              }
                .into_any()
            }
          })
          .into_any()
      }}
    </Suspense>
  }
}
