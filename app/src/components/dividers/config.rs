use crate::prelude::*;

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
#[derive(Clone, Copy)]
pub struct Divider {
  /// Whether to render the center dot (uses `--color-dot` token).
  /// Also controls the `.divider--centered` modifier class.
  pub show_dot: bool,

  /// Gap spacing (left, right) in spacing-scale units (1 unit = 0.25rem = 4px).
  /// Injected as `--divider-gap-left` / `--divider-gap-right` CSS custom properties.
  pub dot_pos: Option<(u8, u8)>,

  /// Vertical padding as a CSS length string.
  /// Injected as `--divider-padding`. Default: `"0.5rem"` (sp(2) equivalent).
  pub padding: &'static str,
}

impl Default for Divider {
  fn default() -> Self {
    Self {
      show_dot: false,
      dot_pos: None,
      padding: "0.5rem", // sp(2) — SCSS default matches via var(--divider-padding, #{sp(2)})
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
  pub fn default_with_dot() -> Self {
    Self::default().with_dot().with_dot_pos(4, 4)
  }

  /// Enables the center dot and the `.divider--centered` modifier.
  ///
  /// Automatically sets symmetric 6-unit gaps if `with_dot_pos` has not
  /// already been called.
  #[must_use]
  pub const fn with_dot(mut self) -> Self {
    self.show_dot = true;
    if self.dot_pos.is_none() {
      self.dot_pos = Some((6, 6));
    }
    self
  }

  /// Sets the gap on each side of the dot in spacing-scale units (1 unit = 0.25rem).
  ///
  /// Common values: 2 = 8px, 4 = 16px, 6 = 24px (default when using `with_dot()`).
  #[must_use]
  pub const fn with_dot_pos(mut self, left: u8, right: u8) -> Self {
    self.dot_pos = Some((left, right));
    self
  }

  /// Overrides the vertical padding. Pass any valid CSS length, e.g. `"1.5rem"`.
  #[must_use]
  pub const fn with_padding(mut self, padding: &'static str) -> Self {
    self.padding = padding;
    self
  }
}

/// Converts a spacing-scale unit to a CSS rem string.
/// 1 unit = 0.25rem = 4px (matches the `$spacing` map in base/_tokens.scss).
#[inline]
#[must_use]
pub fn units_to_rem(units: u8) -> String {
  format!("{}rem", f32::from(units) * 0.25)
}
