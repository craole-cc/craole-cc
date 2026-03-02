use super::_prelude::*;

#[component]
pub fn Log() -> impl IntoView {
  let (selected_slug, set_selected_slug,) = signal::<Option<String,>,>(None,);

  // Resource for the list
  let posts = Resource::new(|| (), |()| async move { list_posts().await },);

  // Resource for selected post detail
  let selected_post = Resource::new(
    move || selected_slug.get(),
    |slug_opt| async move {
      match slug_opt {
        | Some(slug,) => get_post_by_slug(slug,).await,
        | None => Ok(None,),
      }
    },
  );

  view! {
    <section class="readable page page--log">
      <header class="page__header">
        <h1 class="page__title">"Log"</h1>
        <p class="page__sub">"CV, blog & things worth writing down."</p>
      </header>

      <Divider />

      <h2>"Posts"</h2>

      <Suspense fallback=move || {
        view! { <p>"Loading posts…"</p> }.into_any()
      }>
        {move || {
          posts
            .get()
            .map(|res| match res {
              Ok(items) if items.is_empty() => view! { <p>"No published posts yet."</p> }.into_any(),
              Ok(items) => {

                view! {
                  <ul class="log-list">
                    {items
                      .into_iter()
                      .map(|p| {
                        let slug = p.slug.clone();
                        let title = p.title.clone();
                        let excerpt = p.excerpt.unwrap_or_default();

                        view! {
                          <li class="log-list__item">
                            // <a
                            //   class="log-list__button"
                            //   href=format!("/log/{}", slug)
                            //   on:click=move |ev| {
                            //     ev.prevent_default();
                            //     set_selected_slug.set(Some(slug.clone()));
                            //   }
                            // >
                              <a class="log-list__button" href=format!("/log/{}", slug)>
                              <div class="log-list__title">{title}</div>
                              <div class="log-list__excerpt">{excerpt}</div>
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

                view! { <p class="error">"Failed to load posts: " {e.to_string()}</p> }
                  .into_any()
              }
            })
            .into_any()
        }}
      </Suspense>

      <Divider />

      <Suspense fallback=move || {
        view! { <p>"Loading…"</p> }.into_any()
      }>
        {move || {
          selected_post
            .get()
            .map(|res| match res {
              Ok(Some(p)) => {
                view! {
                  <article class="post">
                    <h3>{p.title}</h3>
                    <p class="post__meta">
                      {p.kind} " • " {p.published_at.clone().unwrap_or(p.created_at)}
                    </p>
                    <div class="post__body markdown" inner_html=render_markdown(&p.body) />
                  </article>
                }
                  .into_any()
              }
              Ok(None) => {
                view! {
                  <div class="log-empty">
                    <h3 class="log-empty__title">"Nothing selected."</h3>
                    <p class="log-empty__body">
                      "Choose a post to explore ideas, experiments, and documentation."
                    </p>
                  </div>
                }
                  .into_any()
              }
              Err(e) => {

                view! { <p class="error">"Failed to load post: " {e.to_string()}</p> }
                  .into_any()
              }
            })
            .into_any()
        }}
      </Suspense>
    </section>
  }
}
