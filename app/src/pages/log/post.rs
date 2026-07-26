use super::_prelude::*;

const WORDS_PER_MINUTE : usize = 200;

fn estimated_read_time(body : &str,) -> usize {
  let words = body.split_whitespace().count();
  ((words + WORDS_PER_MINUTE - 1) / WORDS_PER_MINUTE).max(1,)
}

fn article_body(body : &str,) -> String {
  let lines = body.lines().collect::<Vec<_,>>();
  let Some(first_index,) = lines.iter().position(|line| !line.trim().is_empty(),) else {
    return String::new();
  };
  let start = if lines[first_index].trim().starts_with("# ",) {
    first_index + 1
  } else {
    first_index
  };
  lines[start ..].join("\n",)
}

#[component]
fn AudioPlayer(slug : String,) -> impl IntoView {
  let source = format!("/audio/{}.mp3", slug,);
  view! {
    <div class="post__audio" aria-label="Article audio">
      <span class="post__audio-label">"Listen to this article"</span>
      <audio class="post__audio-player" controls preload="metadata">
        <source src=source.clone() type="audio/mpeg" />
        "Your browser does not support the audio player."
      </audio>
      <a class="post__audio-download" href=source download>
        "Download audio"
      </a>
    </div>
  }
}

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
      <Suspense fallback=move || {
        view! { <p class="log-loading" aria-busy="true">"Loading post…"</p> }.into_any()
      }>
        {move || {
          post.get().map(|res| match res {
            Ok(Some(p)) => {
              let body = article_body(&p.body,);
              view! {
                <article class="post">
                  {p.cover_url.map(|url| view! {
                    <figure class="post-hero">
                      <img class="post-hero__img" src=url alt=p.title.clone() loading="lazy" decoding="async" />
                    </figure>
                  })}
                  <header class="post__header">
                    <p class="post__meta">
                      {p.kind.clone()} " • " {p.published_at.unwrap_or_else(|| p.created_at.clone())}
                      " • " {estimated_read_time(&body)} " min read"
                    </p>
                    <h1 class="post__title">{p.title}</h1>
                    <AudioPlayer slug=p.slug.clone() />
                  </header>
                  <div class="post__body markdown" inner_html=render_markdown(&body) />
                  <footer class="post__footer">
                    <a class="post-nav__back" href="/log">"← Back to Log"</a>
                  </footer>
                </article>
              }.into_any()
            }
            Ok(None) => view! {
              <div class="log-empty">
                <h2 class="log-empty__title">"Post not found."</h2>
                <p class="log-empty__body">"This post may have moved or been removed."</p>
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
          }).into_any()
        }}
      </Suspense>
    </section>
  }
}
