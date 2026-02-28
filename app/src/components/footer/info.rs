use crate::prelude::*;

#[component]
pub fn Copyright() -> impl IntoView {
  view! {
    <div class="footer__copyright">
      <p class="footer__name">
        <span>{AUTHOR_FIRSTNAME}</span>
        <span class="footer__alias">" "{AUTHOR_ALIAS}" "</span>
        <span>{AUTHOR_SURNAME}</span>
      </p>
      <div class="footer__meta">
        <p>
          "Built with "
          <a
            href="https://www.rust-lang.org"
            target="_blank"
            rel="noopener noreferrer"
            class="footer__tech footer__tech--rust"
          >
            "Rust"
          </a> " & "
          <a
            href="https://leptos.dev"
            target="_blank"
            rel="noopener noreferrer"
            class="footer__tech footer__tech--leptos"
          >
            "Leptos"
          </a>
        </p>
        <p>"© "{COPYRIGHT_YEAR}" — All rights reserved"</p>
      </div>
    </div>
  }
}
