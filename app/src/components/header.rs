use crate::prelude::{
  icons::{
    menu_closed,
    menu_open,
  },
  *,
};

// ═══════════════════════════════════════════════════════════════════════════════
// header.rs — Site navigation
// ───────────────────────────────────────────────────────────────────────────────
//
// All pages are always shown. The current page gets `site-nav__link--active`
// so users always know where they are. Width stays stable; no layout shift.
// ═══════════════════════════════════════════════════════════════════════════════

#[component]
fn NavLogo() -> impl IntoView {
  view! {
    <a href="/" class="site-nav__logo" aria-label="Craole.CC — Home">
      <span class="site-nav__logo-mark" aria-hidden="true">
        "CC"
      </span>
      <span class="site-nav__logo-name">"Craole.CC"</span>
    </a>
  }
}

#[component]
fn NavLinks(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
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

#[component]
fn Hamburger(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <button
      class="site-nav__menu-btn"
      aria-label=move || if open.get() { "Close menu" } else { "Open menu" }
      aria-expanded=move || open.get().to_string()
      aria-controls="primary-nav"
      on:click=move |_| set_open.update(|v| *v = !*v)
    >
      {move || {
        let icon = if open.get() { menu_open::default() } else { menu_closed::default() };
        view! { <IconRender icon /> }
      }}
    </button>
  }
}

#[component]
fn NavControls(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <div class="site-nav__controls">
      <ThemeSwitcher />
      <Hamburger open set_open />
    </div>
  }
}

#[component]
pub fn Header() -> impl IntoView {
  #[allow(unused_variables)]
  let (scrolled, set_scrolled,) = signal(false,);

  #[cfg(feature = "hydrate")]
  Effect::new(move |_| {
    let Some(vista,) = web_sys::window() else {
      return;
    };
    let handler = Closure::wrap(Box::new(move || {
      let y = window().and_then(|w| w.scroll_y().ok(),).unwrap_or(0.0,);
      set_scrolled.set(y > 60.0,);
    },) as Box<dyn Fn(),>,);
    let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
    let _ = vista.add_event_listener_with_callback("scroll", &cb,);
    handler.forget();
  },);

  let (open, set_open,) = signal(false,);

  view! {
    <header class=move || {
      if scrolled.get() { "site-nav site-nav--scrolled" } else { "site-nav" }
    }>
      <div class="site-nav__inner">
        <NavLogo />
        <NavLinks open set_open />
        <NavControls open set_open />
      </div>
    </header>
  }
}
