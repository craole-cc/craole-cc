use super::_prelude::*;

#[component]
pub fn Post() -> impl IntoView {
  let params = use_params_map();
  let slug = move || params.with(|p| p.get("slug",).unwrap_or_default(),);
  let post = Resource::new(slug, |slug| async move {
    if slug.is_empty() {
      return Ok::<Option<Post,>, ServerFnError,>(None,);
    }
    get_post_by_slug(slug,).await
  },);

  view! {
    <section class="readable page page--log-post">
      <Suspense fallback=move || {
        view! { <p>"Loading…"</p> }.into_any()
      }>
        {move || {
          post
            .get()
            .map(|res| match res {
              Ok(Some(p)) => {
                view! {
                  <article class="post">
                    <header class="post__header">
                      <h1 class="post__title">{p.title}</h1>
                      <p class="post__meta">
                        {p.kind} " • " {p.published_at.clone().unwrap_or(p.created_at)}
                      </p>
                    </header>
                    <div class="post__body markdown" inner_html=render_markdown(&p.body) />
                  </article>
                }
                  .into_any()
              }
              Ok(None) => {

                view! { <p>"Post not found."</p> }
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
