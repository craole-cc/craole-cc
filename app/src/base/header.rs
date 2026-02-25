use crate::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════════
// header.rs — Site navigation
// ───────────────────────────────────────────────────────────────────────────────
//
// SUB-COMPONENTS
//   Header      — root orchestrator; owns scrolled + open signals
//   NavLogo     — logo badge + wordmark anchor
//   NavLinks    — mobile drawer / desktop inline link list
//   NavControls — right-side cluster (ThemeSwitcher + hamburger)
//   Hamburger   — icon-based open/close button via registry
//
// SIGNAL PASSING
//   `scrolled` and `open` are owned by Header and passed down as ReadSignal
//   so each child can react to them without owning mutation rights.
//   Only NavControls receives WriteSignal<bool> for `open`.
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Logo ──────────────────────────────────────────────────────────────────────

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

// ── Link list ─────────────────────────────────────────────────────────────────

#[component]
fn NavLinks(
  /// Whether the mobile drawer is open.
  open : ReadSignal<bool,>,
  /// Callback so clicking a link closes the drawer.
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

// ── Hamburger ─────────────────────────────────────────────────────────────────

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
        let icon = if open.get() { Icons::MenuOpen.get() } else { Icons::MenuClosed.get() };
        view! { <IconRender icon /> }
      }}
    </button>
  }
}

// ── Controls cluster ──────────────────────────────────────────────────────────

#[component]
fn NavControls(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <div class="site-nav__controls">
      <ThemeSwitcher />
      <Hamburger open set_open />
    </div>
  }
}

// ── Root ──────────────────────────────────────────────────────────────────────

#[component]
pub fn Header() -> impl IntoView {
  // ── Scroll detection ──────────────────────────────────────────────────────
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

  // ── Mobile drawer ─────────────────────────────────────────────────────────
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
