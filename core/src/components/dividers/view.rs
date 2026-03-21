#![allow(clippy::must_use_candidate)]

use {
  super::config::{
    Divider,
    units_to_rem,
  },
  crate::prelude::*,
};

/// Flexible horizontal divider with optional centered dot accent.
///
/// All layout, sizing, and colour is SCSS-driven via BEM classes and tokens.
/// The only inline styles are the CSS custom properties for gap widths and
/// padding, which cannot be expressed as static class names.
///
/// # Props
///
/// * `config` — [`Divider`] (optional — uses [`Divider::default`] if omitted)
#[component]
pub fn Divider(
  /// Divider configuration (optional — uses [`Divider::default`] if omitted)
  #[prop(default = Divider::default())]
  config : Divider,
) -> impl IntoView {
  let (left_gap, right_gap,) = config.dot_pos.unwrap_or((0, 0,),);

  // Only --divider-padding and --divider-gap-* are injected as inline custom
  // properties. All other layout (margin, max-width) is handled by the
  // .divider--centered modifier in SCSS.
  let style = format!(
    "--divider-padding: {}; --divider-gap-left: {}; --divider-gap-right: {};",
    config.padding,
    units_to_rem(left_gap,),
    units_to_rem(right_gap,),
  );

  // .divider--centered is applied when a dot is shown — matches the SCSS rule
  // that constrains max-width and centres the divider in its parent.
  let class = if config.show_dot {
    "divider divider--centered"
  } else {
    "divider"
  };

  view! {
    <div class=class style=style>
      // ~@ Left line — fades in from transparent
      <div class="divider__line divider__line--left" />

      // ~@ Left spacer (zero width when no dot)
      <div class="divider__spacer divider__spacer--left" />

      // ~@ Dot
      {config.show_dot.then(|| view! { <span class="divider__dot" /> })}

      // ~@ Right spacer (zero width when no dot)
      <div class="divider__spacer divider__spacer--right" />

      // ~@ Right line — fades out to transparent
      <div class="divider__line divider__line--right" />
    </div>
  }
}
