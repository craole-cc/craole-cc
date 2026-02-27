use crate::_prelude::{
  icons::*,
  *,
};

// ═══════════════════════════════════════════════════════════════════════════════
// header.rs — Site navigation
// ───────────────────────────────────────────────────────────────────────────────
//
// SUB-COMPONENTS
//   Header      — root orchestrator; owns `scrolled` and `open` signals
//   NavLogo     — logo badge + wordmark anchor
//   NavLinks    — mobile drawer / desktop inline link list
//   NavControls — right-side cluster (ThemeSwitcher + Hamburger)
//   Hamburger   — icon-based open/close toggle using registry free functions
//
// SIGNAL PASSING
//   `scrolled` and `open` are owned by Header and passed down as ReadSignal
//   so each child can react without owning mutation rights.
//   Only NavControls receives WriteSignal<bool> for `open`.
//
// ICONS
//   menu_closed::default() and menu_open::default() are resolved at render
//   time via the registry free functions — no Icons enum dispatch needed.
//
// ═══════════════════════════════════════════════════════════════════════════════

//╔═══════════════════════════════════════════════════════════╗
//║ Logo                                                      ║
//╚═══════════════════════════════════════════════════════════╝

/// Renders the logo badge and wordmark as a home anchor.
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

//╔═══════════════════════════════════════════════════════════╗
//║ Site Navigation - Link List                               ║
//╚═══════════════════════════════════════════════════════════╝

/// Renders the primary navigation link list.
///
/// Adds `site-nav__links--open` when the mobile drawer is open.
/// Clicking any link closes the drawer via `set_open`.
#[component]
fn NavLinks(
  /// Whether the mobile drawer is currently open.
  open : ReadSignal<bool,>,
  /// Closes the drawer when a link is clicked.
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  view! {
    <nav
      id="primary-nav"
      class=move || {
        if open.get() { "site-nav__links site-nav__links--open" } else { "site-nav__links" }
      }
      aria-label="Primary"
    >
      {FACETS
        .iter()
        .map(|facet| {
          view! {
            <a
              href=format!("/{}", facet.slug)
              class="site-nav__link"
              title=facet.description
              on:click=move |_| set_open.set(false)
            >
              {facet.label}
            </a>
          }
        })
        .collect::<Vec<_>>()}
    </nav>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Site Navigation  - Hamburger                              ║
//╚═══════════════════════════════════════════════════════════╝
// Reactively swaps between menu_open::default() and menu_closed::default()
// from the registry. Icons inherit currentColor from the button CSS —
// no .colored() call needed here.

/// Icon-based toggle button for the mobile navigation drawer.
///
/// Renders [`menu_open::default`] when the drawer is open and
/// [`menu_closed::default`] when it is closed.
#[component]
fn Hamburger(
  /// Whether the mobile drawer is currently open.
  open : ReadSignal<bool,>,
  /// Toggles the drawer open/closed.
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
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

//╔═══════════════════════════════════════════════════════════╗
//║ Controls Cluster                                          ║
//╚═══════════════════════════════════════════════════════════╝

/// Right-side control cluster containing the theme switcher and hamburger.
#[component]
fn NavControls(
  /// Whether the mobile drawer is currently open.
  open : ReadSignal<bool,>,
  /// Passed through to [`Hamburger`] for toggling the drawer.
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  view! {
    <div class="site-nav__controls">
      <ThemeSwitcher />
      <Hamburger open set_open />
    </div>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Root                                                      ║
//╚═══════════════════════════════════════════════════════════╝

/// Site-wide header with scroll detection and a mobile navigation drawer.
///
/// Adds `site-nav--scrolled` to the `<header>` element once the page has
/// scrolled past 60 px, enabling CSS-driven compact/shadow transitions.
#[component]
pub fn Header() -> impl IntoView {
  // ── Scroll detection ────────────────────────────────────────────────────
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

  // ── Mobile drawer ────────────────────────────────────────────────────────
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
