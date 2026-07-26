use {
  crate::prelude::*,
  std::{
    cell::Cell,
    rc::Rc,
    time::Duration,
  },
};

const SLIDES : &[&str] = &[
  // Bridges
  "https://images.unsplash.com/photo-1433086966358-54859d0ed716?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1447752875215-b2761acb3c5d?auto=format&fit=crop&w=1920&q=80",
  // Beaches
  "https://images.unsplash.com/photo-1507525428034-b723cf961d3e?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1519046904884-53103b34b206?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1559128010-7c1ad6e1b6a5?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1552465011-b4e21bf6e79a?auto=format&fit=crop&w=1920&q=80",
  // Rivers & Waterfalls
  "https://images.unsplash.com/photo-1546587348-d12660c30c50?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1586348943529-beaae6c28db9?auto=format&fit=crop&w=1920&q=80",
  // Mountains
  "https://images.unsplash.com/photo-1501854140801-50d01698950b?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1469474968028-56623f02e42e?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1464822759023-fed622ff2c3b?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1500534314209-a25ddb2bd429?auto=format&fit=crop&w=1920&q=80",
  // Ruins & Temples
  "https://images.unsplash.com/photo-1555400038-63f5ba517a47?auto=format&fit=crop&w=1920&q=80",
];

const SLIDE_SECS : f64 = 5.0;

#[component]
pub fn Hero() -> impl IntoView {
  #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
  let ThemeContext { set_hue, .. } = expect_context::<ThemeContext,>();

  // scrolled signal exists in all builds — only the Effect is hydrate-only
  let (scrolled, set_scrolled,) = signal(false,);

  #[cfg(feature = "hydrate")]
  {
    Effect::new(move |_| {
      let handler = Closure::<dyn Fn(),>::wrap(Box::new(move || {
        let y = window().and_then(|w| w.scroll_y().ok(),).unwrap_or(0.0,);
        set_scrolled.set(y > 50.0,);
      },) as Box<dyn Fn(),>,);
      let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
      let _ = window().map(|w| w.add_event_listener_with_callback("scroll", &cb,),);
      handler.forget();
    },);

    let url = SLIDES[0];
    extract_hue_from_url(url, move |hue| set_hue.set(hue,),);

    let slide_index = Rc::new(Cell::new(1_usize,),);
    set_interval(
      move || {
        let i = slide_index.get() % SLIDES.len();
        slide_index.set(i + 1,);
        let url = SLIDES[i];
        extract_hue_from_url(url, move |hue| set_hue.set(hue,),);
      },
      Duration::from_secs_f64(SLIDE_SECS,),
    );
  }

  view! {
    <section class=move || if scrolled.get() { "hero hero--scrolled" } else { "hero" }>
      <figure class="hero__backdrop" aria-hidden="true">
        {SLIDES
          .iter()
          .zip((0u32..).map(|i| f64::from(i) * SLIDE_SECS))
          .map(|(src, delay)| {
            view! {
              <span
                class="hero__slide"
                style=format!("background-image:url('{src}');animation-delay:{delay}s")
              />
            }
          })
          .collect_view()}
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
