use crate::prelude::*;

/// All site pages as footer navigation links.
#[component]
pub fn PageNav() -> impl IntoView {
  view! {
    <nav class="footer__pages">
      {PAGES
        .iter()
        .enumerate()
        .map(|(i, page)| {
          let divided = i < PAGES.len() - 1;
          view! {
            <a
              href=page.path
              title=page.description
              class=if divided { "footer__page footer__page--divided" } else { "footer__page" }
            >
              {page.label}
            </a>
          }
        })
        .collect_view()}
    </nav>
  }
}
