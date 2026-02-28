use crate::prelude::*;

#[component]
pub fn Log() -> impl IntoView {
  view! {
    <section class="readable page page--log">
      <header class="page__header">
        <h1 class="page__title">"Log"</h1>
        <p class="page__sub">"CV, blog & things worth writing down."</p>
      </header>
      <Divider />
      <p class="page__placeholder">"Entries coming soon — check back shortly."</p>
    </section>
  }
}
