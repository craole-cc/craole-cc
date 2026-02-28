use {
  super::{
    Controls,
    Logo,
    PageNav,
  },
  crate::prelude::*,
};

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
        <Logo />
        <PageNav open set_open />
        <Controls open set_open />
      </div>
    </header>
  }
}
