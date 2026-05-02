#![allow(clippy::must_use_candidate)]
use crate::prelude::*;

/// Floating "back to top" button that appears once the user scrolls
/// past a threshold (default 400 px). Smoothly scrolls back to the top
/// when clicked, then hides again.
#[component]
pub fn BackToTop(
  /// Vertical scroll distance (in pixels) before the button appears.
  #[prop(default = 400.0)]
  threshold: f64,
) -> impl IntoView {
  let (visible, set_visible) = signal(false);

  // Listen to window scroll events and toggle visibility
  Effect::new(move |_| {
    if let Some(win) = window() {
      let cb = Closure::<dyn Fn()>::new(move || {
        let y = window().map_or(0.0, |w| w.scroll_y().unwrap_or(0.0));
        set_visible.set(y > threshold);
      });

      let _ = win.add_event_listener_with_callback("scroll", cb.as_ref().unchecked_ref());

      // Keep the closure alive for the lifetime of the component
      cb.forget();
    }
  });

  let on_click = move |_: MouseEvent| {
    if let Some(win) = window() {
      let opts = ScrollToOptions::new();
      opts.set_top(0.0);
      opts.set_behavior(ScrollBehavior::Smooth);
      win.scroll_to_with_scroll_to_options(&opts);
    }
  };

  view! {
    <button
      class="back-to-top"
      class:back-to-top--visible=move || visible.get()
      on:click=on_click
      aria-label="Scroll to top"
      title="Back to top"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <polyline points="18 15 12 9 6 15" />
      </svg>
    </button>
  }
}
