use super::_prelude::*;

#[component]
pub fn Featured(
  first : Option<PostSummary,>,
  second : Option<PostSummary,>,
  third : Option<PostSummary,>,
) -> impl IntoView {
  if first.is_none() && second.is_none() {
    return Either::Left((),);
  }

  Either::Right(view! {
    <section class="log-featured readable">
      <div class="log-featured__grid">

        // -- Left column ─────────────────────────────────────────────────────
        <div class="log-featured__left">
          {first
            .map(|p| {
              let date = p.published_at.unwrap_or(p.created_at);
              view! {
                <article class="log-featured__primary">
                  <a href=format!(
                    "/log/{}",
                    p.slug,
                  )>
                    {p
                      .cover_url
                      .map(|url| {
                        view! {
                          <figure class="log-featured__cover log-featured__cover--wide">
                            <img src=url alt=p.title.clone() loading="lazy" decoding="async" />
                          </figure>
                        }
                      })} <footer class="log-featured__body">
                      <span class="log-featured__meta">{p.kind} " — " {date}</span>
                      <h2 class="log-featured__title">{p.title}</h2>
                      {p.excerpt.map(|e| view! { <p class="log-featured__excerpt">{e}</p> })}
                    </footer>
                  </a>
                </article>
              }
            })}
          {third
            .map(|p| {
              let date = p.published_at.unwrap_or(p.created_at);
              view! {
                <article class="log-featured__tertiary">
                  <a href=format!(
                    "/log/{}",
                    p.slug,
                  )>
                    {p
                      .cover_url
                      .map(|url| {
                        view! {
                          <figure class="log-featured__cover log-featured__cover--thin">
                            <img src=url alt=p.title.clone() loading="lazy" decoding="async" />
                          </figure>
                        }
                      })} <footer class="log-featured__body">
                      <span class="log-featured__meta">{p.kind} " — " {date}</span>
                      <h2 class="log-featured__title log-featured__title--sm">{p.title}</h2>
                      {p.excerpt.map(|e| view! { <p class="log-featured__excerpt">{e}</p> })}
                    </footer>
                  </a>
                </article>
              }
            })}
        </div>

        // -- Right column ────────────────────────────────────────────────────
        <div class="log-featured__right">
          {second
            .map(|p| {
              let date = p.published_at.unwrap_or(p.created_at);
              view! {
                <article class="log-featured__secondary">
                  <a href=format!(
                    "/log/{}",
                    p.slug,
                  )>
                    {p
                      .cover_url
                      .map(|url| {
                        view! {
                          <figure class="log-featured__cover log-featured__cover--portrait">
                            <img src=url alt=p.title.clone() loading="lazy" decoding="async" />
                          </figure>
                        }
                      })} <footer class="log-featured__body">
                      <span class="log-featured__meta">{p.kind} " — " {date}</span>
                      <h2 class="log-featured__title log-featured__title--sm">{p.title}</h2>
                      {p.excerpt.map(|e| view! { <p class="log-featured__excerpt">{e}</p> })}
                    </footer>
                  </a>
                </article>
              }
            })}
        </div>

      </div>
    </section>
  },)
}
