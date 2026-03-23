use super::_prelude::*;

#[component]
pub fn Header() -> impl IntoView {
  view! {
    <header class="dev-header readable">
      <span class="dev-header__label">"The Workshop"</span>
      <h1 class="dev-header__title">"Dev"</h1>
      <p class="dev-header__sub">
        "Personal projects, open-source tools, and systems built with Rust and friends."
      </p>
    </header>
  }
}
