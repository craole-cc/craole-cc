use {
  crate::prelude::*,
};

// Keep the first-paint hero to one responsive image. The previous rotating
// backdrop declared fourteen external 1920px images, causing mobile browsers
// to fetch far more imagery than was visible at once.
const HERO_IMAGE: &str =
  "https://images.unsplash.com/photo-1433086966358-54859d0ed716?auto=format&fit=crop&w=1200&q=60";

#[component]
pub fn Hero() -> impl IntoView {
  #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
  let ThemeContext { set_hue, .. } = expect_context::<ThemeContext>();

  let (scrolled, set_scrolled) = signal(false);

  #[cfg(feature = "hydrate")]
  {
    Effect::new(move |_| {
      let handler = Closure::<dyn Fn()>::wrap(Box::new(move || {
        let y = window().and_then(|w| w.scroll_y().ok()).unwrap_or(0.0);
        set_scrolled.set(y > 50.0);
      }) as Box<dyn Fn()>);
      let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
      let _ = window().map(|w| w.add_event_listener_with_callback("scroll", &cb));
      handler.forget();
    });

    extract_hue_from_url(HERO_IMAGE, move |hue| set_hue.set(hue));
  }

  view! {
    <section class=move || if scrolled.get() { "hero hero--scrolled" } else { "hero" }>
      <figure class="hero__backdrop" aria-hidden="true">
        <span
          class="hero__slide"
          style=format!("background-image:url('{HERO_IMAGE}')")
        />
        <span class="hero__scrim" />
        <span class="hero__noise" />
      </figure>

      <article class="hero__content">
        <h1 class="hero__headline">
          <span>{AUTHOR_FIRSTNAME}" "</span>
          <em>{AUTHOR_ALIAS}</em>
          <span>" "{AUTHOR_SURNAME}</span>
        </h1>

        <p class="hero__sub">"Creative engineering & visual narrative"</p>
      </article>

      <div class="hero__scroll" aria-hidden="true">
        <span class="hero__scroll-label">"SCROLL"</span>
        <span class="hero__scroll-line" />
      </div>
    </section>
  }
}
