use super::_prelude::*;

#[component]
pub fn Archive(posts : Vec<PostSummary,>,) -> impl IntoView {
  if posts.is_empty() {
    return Either::Left((),);
  }

  Either::Right(view! {
    <section class="log-archive">
      <h3 class="log-archive__heading">"Recent Archives"</h3>
      <ul class="log-archive__list" role="list">
        {posts
          .into_iter()
          .map(|p| {
            let date = p.published_at.unwrap_or(p.created_at);
            view! {
              <li class="log-archive__item">
                <a href=format!("/log/{}", p.slug) class="log-archive__link">
                  <span class="log-archive__date">{date}</span>
                  <div class="log-archive__body">
                    <h4 class="log-archive__title">{p.title}</h4>
                    {p.excerpt.map(|e| view! { <p class="log-archive__excerpt">{e}</p> })}
                    <div class="log-archive__footer">
                      <span class="log-archive__kind">{p.kind}</span>
                      <span class="log-archive__read">"Read Journal Entry →"</span>
                    </div>
                  </div>
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </section>
  },)
}
