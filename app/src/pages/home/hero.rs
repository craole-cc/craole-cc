use {
  crate::prelude::*,
  wasm_bindgen::{
    JsCast,
    closure::Closure,
  },
};

//╔═══════════════════════════════════════════════════════════╗
//║ Slideshow                                                 ║
//╚═══════════════════════════════════════════════════════════╝

const SLIDES : &[&str] = &[
  "https://images.unsplash.com/photo-1506905925346-21bda4d32df4?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1469474968028-56623f02e42e?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1447752875215-b2761acb3c5d?auto=format&fit=crop&w=1920&q=80",
  "https://images.unsplash.com/photo-1433086966358-54859d0ed716?auto=format&fit=crop&w=1920&q=80",
];

const SLIDE_SECS : f64 = 5.0;

//╔═══════════════════════════════════════════════════════════╗
//║ Component                                                 ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn Hero() -> impl IntoView {
  // Pull set_hue out of the global ThemeContext
  let ThemeContext { set_hue, .. } = expect_context::<ThemeContext,>();

  // On mount: schedule hue extraction for each slide, staggered to
  // fire in sync with the CSS crossfade animation-delay.
  Effect::new(move |_| {
    for (i, &url,) in SLIDES.iter().enumerate() {
      let delay_ms = (i as f64 * SLIDE_SECS * 1000.0) as i32;

      let handle = Closure::once(Box::new(move || {
        extract_hue_from_url(url, move |hue| set_hue.set(hue,),);
      },) as Box<dyn FnOnce(),>,);

      if let Some(win,) = web_sys::window() {
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
          handle.as_ref().unchecked_ref(),
          delay_ms,
        );
      }
      handle.forget();
    }
  },);

  view! {
    <section class="hero">

      <figure class="hero__backdrop" aria-hidden="true">
        {SLIDES
          .iter()
          .enumerate()
          .map(|(i, src)| {
            view! {
              <span
                class="hero__slide"
                style=format!(
                  "background-image:url('{}');animation-delay:{}s",
                  src,
                  i as f64 * SLIDE_SECS,
                )
              />
            }
          })
          .collect::<Vec<_>>()}
        <span class="hero__scrim" />
        <span class="hero__noise" />
      </figure>

      <article class="hero__content">

        <p class="hero__badge" role="note">
          <span class="hero__badge-dot" aria-hidden="true" />
          "Available for freelance work"
        </p>

        <h1 class="hero__headline">
          "Building " <em>"systems,"</em> <br /> "capturing " <em>"moments,"</em> <br />
          "expressing " <em>"ideas."</em>
        </h1>

        <p class="hero__sub">"Software Engineer · Data Specialist · TEFL Educator · Artist"</p>

        <nav class="hero__ctas" aria-label="Call to action">
          <a href="/code" class="hero__cta-primary">
            "View my work"
            <svg
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="2"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"
              />
            </svg>
          </a>
          <a href="/life" class="hero__cta-ghost">
            "Explore life"
          </a>
        </nav>

        <p class="hero__scroll" aria-hidden="true">
          <span class="hero__scroll-line" />
          <small>"scroll"</small>
        </p>

      </article>

      <aside class="hero__stats" aria-label="Quick facts">
        <dl>
          <div class="hero__stat">
            <dt>"Years coding"</dt>
            <dd>"10+"</dd>
          </div>
          <div class="hero__stat">
            <dt>"Disciplines"</dt>
            <dd>"4"</dd>
          </div>
          <div class="hero__stat">
            <dt>"Curiosity"</dt>
            <dd>"∞"</dd>
          </div>
        </dl>
      </aside>

    </section>
  }
}
