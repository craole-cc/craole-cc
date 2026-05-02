use {super::card::Card, crate::prelude::*};

#[component]
pub fn Mosaic(items: Vec<Media>) -> impl IntoView {
  if items.is_empty() {
    return Either::Left(view! { <p class="art-empty readable">"Nothing here yet."</p> });
  }

  Either::Right(view! {
    <section class="art-mosaic" aria-label="Photo gallery">
      {items.into_iter().map(|item| view! { <Card item=item /> }).collect_view()}
    </section>
  })
}
