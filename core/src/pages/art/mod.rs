use crate::prelude::*;

#[component]
pub fn Art() -> impl IntoView {
  view! {
    <section class="readable page page--art">
      <header class="page__header">
        <h1 class="page__title">"Art"</h1>
        <p class="page__sub">"Photography, music & video — expression through every medium."</p>
      </header>
      <Divider />
      <p class="page__placeholder">"Gallery coming soon — check back shortly."</p>
    </section>
  }
}
