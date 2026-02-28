use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Configuration                                             ║
//╚═══════════════════════════════════════════════════════════╝

/// Configuration for a [`Divider`] component.
///
/// Built via the builder pattern — all methods consume and return `Self`.
///
/// # Examples
///
/// ```rust
/// // Plain full-width line
/// Divider::default()
///
/// // Centered line with accent dot
/// Divider::default_with_dot()
///
/// // Dot with custom gap
/// Divider::default_with_dot().with_dot_pos(2, 8)
///
/// // Custom padding
/// Divider::default().with_padding("1.5rem")
/// ```
#[derive(Clone, Copy,)]
pub struct Divider {
  /// Whether to render the center dot (uses `--color-dot` token).
  /// Also controls the `.divider--centered` modifier class.
  pub show_dot : bool,

  /// Gap spacing (left, right) in spacing-scale units (1 unit = 0.25rem = 4px).
  /// Injected as `--divider-gap-left` / `--divider-gap-right` CSS custom properties.
  pub dot_pos : Option<(u8, u8,),>,

  /// Vertical padding as a CSS length string.
  /// Injected as `--divider-padding`. Default: `"0.5rem"` (sp(2) equivalent).
  pub padding : &'static str,
}

impl Default for Divider {
  fn default() -> Self {
    Self {
      show_dot : false,
      dot_pos :  None,
      padding :  "0.5rem", // sp(2) — SCSS default matches via var(--divider-padding, #{sp(2)})
    }
  }
}

impl Divider {
  /// Centered divider with the default accent dot.
  ///
  /// Adds `.divider--centered` which applies `margin-inline: auto; max-width: 36rem`
  /// via SCSS — no raw margin CSS is injected from Rust.
  ///
  /// Equivalent to:
  /// ```rust
  /// Divider::default().with_dot().with_dot_pos(4, 4,) 
  /// ```
  #[must_use]
  pub fn default_with_dot() -> Self { Self::default().with_dot().with_dot_pos(4, 4,) }

  /// Enables the center dot and the `.divider--centered` modifier.
  ///
  /// Automatically sets symmetric 6-unit gaps if `with_dot_pos` has not
  /// already been called.
  #[must_use]
  pub const fn with_dot(mut self,) -> Self {
    self.show_dot = true;
    if self.dot_pos.is_none() {
      self.dot_pos = Some((6, 6,),);
    }
    self
  }

  /// Sets the gap on each side of the dot in spacing-scale units (1 unit = 0.25rem).
  ///
  /// Common values: 2 = 8px, 4 = 16px, 6 = 24px (default when using `with_dot()`).
  #[must_use]
  pub const fn with_dot_pos(mut self, left : u8, right : u8,) -> Self {
    self.dot_pos = Some((left, right,),);
    self
  }

  /// Overrides the vertical padding. Pass any valid CSS length, e.g. `"1.5rem"`.
  #[must_use]
  pub const fn with_padding(mut self, padding : &'static str,) -> Self {
    self.padding = padding;
    self
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Helpers                                                   ║
//╚═══════════════════════════════════════════════════════════╝

/// Converts a spacing-scale unit to a CSS rem string.
/// 1 unit = 0.25rem = 4px (matches the `$spacing` map in base/_tokens.scss).
#[inline]
fn units_to_rem(units : u8,) -> String { format!("{}rem", f32::from(units) * 0.25) }

//╔═══════════════════════════════════════════════════════════╗
//║ Component                                                 ║
//╚═══════════════════════════════════════════════════════════╝

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
