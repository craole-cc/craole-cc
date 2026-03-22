use crate::prelude::*;

#[component]
pub fn Card(item : Media,) -> impl IntoView {
  view! {
    <figure class="art-card">
      <a href=format!("/art/{}", item.slug) class="art-card__link">
        <img
          class="art-card__img"
          src=item.file_path.clone()
          alt=item.alt_text.clone()
          loading="lazy"
          decoding="async"
        />
        <figcaption class="art-card__overlay">
          <span class="art-card__title">{item.title}</span>
          {item.caption.map(|c| view! { <span class="art-card__caption">{c}</span> })}
          {(!item.tags.is_empty())
            .then(|| {
              view! {
                <ul class="art-card__tags" role="list">
                  {item
                    .tags
                    .iter()
                    .map(|t| view! { <li class="art-card__tag">{t.clone()}</li> })
                    .collect_view()}
                </ul>
              }
            })}
        </figcaption>
      </a>
    </figure>
  }
}
