use crate::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
  view! {
    <header class="art-header readable">
      <span class="art-header__label">"The Gallery"</span>
      <h1 class="art-header__title">"Art"</h1>
      <p class="art-header__sub">
        "Photography, music & video — expression through every medium."
      </p>
    </header>
  }
}
