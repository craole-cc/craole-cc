use crate::prelude::*;

#[component]
pub fn Logo() -> impl IntoView {
  view! {
    <a href="/" class="site-nav__logo" aria-label="Craole.CC — Home">
      <span class="site-nav__logo-mark" aria-hidden="true">
        "CC"
      </span>
      <span class="site-nav__logo-name">"Craole.CC"</span>
    </a>
  }
}
