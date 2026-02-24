use crate::prelude::*;

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

  #[cfg(feature = "hydrate")]
  Effect::new(move |_| {
    for (i, &url,) in SLIDES.iter().enumerate() {
      let delay_ms = (i as f64 * SLIDE_SECS * 1000.0) as i32;
      extract_hue_from_url(url, move |hue| {
        set_timeout(
          move || set_hue.set(hue,),
          std::time::Duration::from_millis(delay_ms as u64,),
        );
      },);
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
