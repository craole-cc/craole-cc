use super::_prelude::*;

#[component]
pub fn Grid(items: Vec<Project>) -> impl IntoView {
  if items.is_empty() {
    return view! {
      <div class="dev-empty readable">
        <h2 class="dev-empty__title">"Workshop is warming up."</h2>
        <p class="dev-empty__body">"Projects are being prepared — check back soon."</p>
      </div>
    }
    .into_any();
  }

  // Split into featured (first) and the rest
  let (featured, rest): (Vec<_>, Vec<_>) = items.into_iter().partition(|p| p.featured);

  view! {
    <div class="dev-grid readable">
      // Featured projects get full-width cards
      {if featured.is_empty() {
        None
      } else {
        Some(
          view! {
            <section class="dev-grid__featured">
              {featured.into_iter().map(|p| view! { <Card project=p /> }).collect_view()}
            </section>
          },
        )
      }} // Remaining projects in a responsive grid
      {if rest.is_empty() {
        None
      } else {
        Some(
          view! {
            <section class="dev-grid__rest">
              {rest.into_iter().map(|p| view! { <Card project=p /> }).collect_view()}
            </section>
          },
        )
      }}
    </div>
  }
  .into_any()
}
