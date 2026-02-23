use {
  crate::prelude::*,
  wasm_bindgen::{
    JsCast,
    closure::Closure,
  },
  web_sys::js_sys,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Navigation                                                ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn Header() -> impl IntoView {
  let (scrolled, set_scrolled,) = signal(false,);

  Effect::new(move |_| {
    let Some(win,) = web_sys::window() else {
      return;
    };

    let handler = Closure::wrap(Box::new(move || {
      let y = web_sys::window()
        .and_then(|w| w.scroll_y().ok(),)
        .unwrap_or(0.0,);
      set_scrolled.set(y > 60.0,);
    },) as Box<dyn Fn(),>,);

    let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
    let _ = win.add_event_listener_with_callback("scroll", &cb,);

    // Keep closure alive for the lifetime of this effect run.
    // Header never unmounts in a SPA, so this is sufficient.
    handler.forget();
  },);

  let (open, set_open,) = signal(false,);

  view! {
    <header class=move || {
      if scrolled.get() { "site-nav site-nav--scrolled" } else { "site-nav" }
    }>
      <div class="site-nav__inner">

        <a href="/" class="site-nav__logo" aria-label="Craole.CC — Home">
          <span class="site-nav__logo-mark" aria-hidden="true">
            "CC"
          </span>
          <span class="site-nav__logo-name">"Craole.CC"</span>
        </a>

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

        <div class="site-nav__controls">
          <ThemeSwitcher />
          <button
            class="site-nav__menu-btn"
            aria-label=move || if open.get() { "Close menu" } else { "Open menu" }
            aria-expanded=move || open.get().to_string()
            aria-controls="primary-nav"
            on:click=move |_| set_open.update(|v| *v = !*v)
          >
            <span class=move || {
              if open.get() { "site-nav__bar site-nav__bar--open" } else { "site-nav__bar" }
            } />
          </button>
        </div>

      </div>
    </header>
  }
}
