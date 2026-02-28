use crate::prelude::*;

#[component]
pub fn PageNav(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  let location = use_location();

  view! {
    <nav
      id="primary-nav"
      class=move || {
        if open.get() { "site-nav__links site-nav__links--open" } else { "site-nav__links" }
      }
      aria-label="Primary"
    >
      {move || {
        let current = location.pathname.get();
        PAGES
          .iter()
          .map(|page| {
            let active = page.path == current.as_str() || (page.path == "/" && current == "/");
            view! {
              <a
                href=page.path
                title=page.description
                aria-current=if active { Some("page") } else { None }
                class=if active {
                  "site-nav__link site-nav__link--active"
                } else {
                  "site-nav__link"
                }
                on:click=move |_| set_open.set(false)
              >
                {page.label}
              </a>
            }
          })
          .collect_view()
      }}
    </nav>
  }
}
