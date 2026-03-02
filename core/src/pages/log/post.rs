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
      <nav class="post-nav" aria-label="Breadcrumb">
        <a class="post-nav__back" href="/log">"← Back to Log"</a>
      </nav>

      <Suspense fallback=move || view! {
        <p class="log-loading" aria-busy="true">"Loading post…"</p>
      }.into_any()>
        {move || {
          post
            .get()
            .map(|res| match res {
              Ok(Some(p)) => view! {
                <article class="post">
                  <header class="post__header">
                    <p class="post__meta">
                      {p.kind.clone()} " • "
                      {p.published_at.unwrap_or_else(|| p.created_at.clone())}
                    </p>
                    <h1 class="post__title">{p.title}</h1>
                  </header>
                  <div class="post__body markdown" inner_html=render_markdown(&p.body) />
                  <footer class="post__footer">
                    <a class="post-nav__back" href="/log">"← Back to Log"</a>
                  </footer>
                </article>
              }.into_any(),
              Ok(None) => view! {
                <div class="log-empty">
                  <h2 class="log-empty__title">"Post not found."</h2>
                  <p class="log-empty__body">
                    "This post may have moved or been removed."
                  </p>
                  <a class="post-nav__back" href="/log">"← Back to Log"</a>
                </div>
              }.into_any(),
              Err(e) => view! {
                <div class="log-empty">
                  <h2 class="log-empty__title">"Something went wrong."</h2>
                  <p class="log-empty__body">{e.to_string()}</p>
                  <a class="post-nav__back" href="/log">"← Back to Log"</a>
                </div>
              }.into_any(),
            })
            .into_any()
        }}
      </Suspense>
    </section>
  }
}

// // database/src/posts.rs - make sure it's not filtering out anything during dev
// pub async fn list_posts(pool : &SqlitePool,) -> Result<Vec<Post,>, sqlx::Error,> {
//   query_as!(
//     Post,
//     r#"
//     SELECT id, title, slug, body, excerpt, kind,
//            published_at, created_at, tags
//     FROM posts
//     ORDER BY created_at DESC
//     "#
//   )
//   .fetch_all(pool,)
//   .await
// }
