use super::_prelude::*;

#[component]
pub fn Hero(
  #[prop(into, optional)] src : Option<String,>,
  #[prop(into, optional)] alt : Option<String,>,
  #[prop(into, optional)] caption : Option<String,>,
) -> impl IntoView {
  match src {
    | None => ().into_view(),
    | Some(src,) => view! {
      <figure class="post-hero">
        <img
          class="post-hero__img"
          src=src
          alt=alt.unwrap_or_default()
          loading="lazy"
          decoding="async"
        />
        {caption.map(|c| view! { <figcaption class="post-hero__caption">{c}</figcaption> })}
      </figure>
    }
    .into_view(),
  }
}
