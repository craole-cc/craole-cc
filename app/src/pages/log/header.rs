use super::_prelude::*;

#[component]
pub fn Header() -> impl IntoView {
  view! {
    <header class="log-header readable">
      <span class="log-header__label">"The Repository"</span>
      <h1 class="log-header__title">"Log"</h1>
      <p class="log-header__sub">
        "A rhythmic record of technical curiosity, creative process, and things worth writing down."
      </p>
    </header>
  }
}
