use crate::prelude::*;

#[component]
pub fn Dev() -> impl IntoView {
  view! {
    <section class="readable page page--dev">
      <header class="page__header">
        <h1 class="page__title">"Dev"</h1>
        <p class="page__sub">
          "Systems, tools & open-source projects built with Rust and friends."
        </p>
      </header>
      <Divider />
      <p class="page__placeholder">"Projects coming soon — check back shortly."</p>
    </section>
  }
}
