#![allow(clippy::must_use_candidate)]
use crate::prelude::*;

#[component]
pub fn PageNav() -> impl IntoView {
  view! {
    <nav class="footer__pages" aria-label="Footer navigation">
      <ul class="footer__page-list">
        {PAGES
          .iter()
          .enumerate()
          .map(|(i, page)| {
            let divided = i < PAGES.len() - 1;
            view! {
              <li>
                <a
                  href=page.path
                  title=page.description
                  class=if divided { "footer__page footer__page--divided" } else { "footer__page" }
                >
                  {page.label}
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </nav>
  }
}
