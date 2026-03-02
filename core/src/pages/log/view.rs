use super::_prelude::*;

#[component]
pub fn Log() -> impl IntoView {
  let posts = Resource::new(|| (), |()| async move { list_posts().await },);

  view! {
    <section class="readable page page--log">
      <header class="page__header">
        <h1 class="page__title">"Log"</h1>
        <p class="page__sub">"CV, blog & things worth writing down."</p>
      </header>

      <Divider />

      <Suspense fallback=move || {
        view! {
          <p class="log-loading" aria-busy="true">
            "Loading posts…"
          </p>
        }
          .into_any()
      }>
        {move || {
          posts
            .get()
            .map(|res| match res {
              Ok(items) if items.is_empty() => {
                view! {
                  <div class="log-empty">
                    <h2 class="log-empty__title">"Nothing here yet."</h2>
                    <p class="log-empty__body">
                      "Posts, experiments, and things worth writing down will appear here."
                    </p>
                  </div>
                }
                  .into_any()
              }
              Ok(items) => {
                view! {
                  <ul class="log-list" role="list">
                    {items
                      .into_iter()
                      .map(|p| {
                        view! {
                          <li class="log-list__item">
                            <a class="log-list__button" href=format!("/log/{}", p.slug)>
                              <span class="log-list__meta">
                                {p.kind.clone()} " • " {p.published_at.unwrap_or(p.created_at)}
                              </span>
                              <span class="log-list__title">{p.title}</span>
                              <span class="log-list__excerpt">{p.excerpt.unwrap_or_default()}</span>
                            </a>
                          </li>
                        }
                      })
                      .collect_view()}
                  </ul>
                }
                  .into_any()
              }
              Err(e) => {
                view! { <p class="error">"Failed to load posts: " {e.to_string()}</p> }.into_any()
              }
            })
            .into_any()
        }}
      </Suspense>
    </section>
  }
}
