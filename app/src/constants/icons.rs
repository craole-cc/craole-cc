//! # Icon & Logo Types
//!
//! Reusable configuration structs for icon and logo rendering.
//! Used anywhere on the site that needs an image with optional class/style/hover state.
//!
//! Style filter constants are in [`crate::constants::colors`]:
//! - [`GREY_FROM_BLACK`] — renders a black SVG as mid-grey
//! - [`DIM_COLOUR`] — dims a colour SVG to match grey icons

//╔═══════════════════════════════════════════════════════════╗
//║ Icon                                                      ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a single icon image.
///
/// Holds the asset path plus optional Tailwind class and inline style strings.
/// Both accessors return `""` when unset, safe to interpolate directly into
/// `class=` and `style=` attributes.
#[derive(Default, Clone)]
pub struct Icon {
  /// Path to the SVG asset (relative to the public root).
  pub src: &'static str,
  /// Optional Tailwind classes (e.g. `"w-6 h-6"`).
  pub class: Option<&'static str>,
  /// Optional inline CSS (e.g. `"filter: brightness(0) invert(0.35);"`).
  pub style: Option<&'static str>,
}

impl Icon {
  /// Creates an icon with the given asset path and no class or style.
  pub fn new(src: &'static str) -> Self {
    Self {
      src,
      ..Default::default()
    }
  }

  /// Sets the Tailwind class string.
  pub fn with_class(mut self, class: &'static str) -> Self {
    self.class = Some(class);
    self
  }

  /// Sets the inline style string.
  pub fn with_style(mut self, style: &'static str) -> Self {
    self.style = Some(style);
    self
  }

  /// Returns the class string, or `""` if unset.
  pub fn class(&self) -> &'static str {
    self.class.unwrap_or_default()
  }

  /// Returns the style string, or `""` if unset.
  pub fn style(&self) -> &'static str {
    self.style.unwrap_or_default()
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Logo                                                      ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a logo with separate idle and hover states.
///
/// When only one image is needed, use [`Logo::new_icon_src`] — hover is
/// initialised as a clone of the reset state.
#[derive(Default, Clone)]
pub struct Logo {
  /// Idle / default icon.
  pub reset: Icon,
  /// Hover icon (defaults to a clone of `reset` if not overridden).
  pub hover: Icon,
}

impl Logo {
  /// Creates an empty logo (both states have empty `src`).
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates a logo where both states share the same `Icon`.
  pub fn new_icon(reset: Icon) -> Self {
    let hover = reset.clone();
    Self { reset, hover }
  }

  /// Creates a logo where both states share the same asset path.
  pub fn new_icon_src(src: &'static str) -> Self {
    Self::new_icon(Icon::new(src))
  }

  /// Replaces the hover state with a fully custom `Icon`.
  pub fn with_hover(mut self, icon: Icon) -> Self {
    self.hover = icon;
    self
  }

  /// Sets the asset path on both reset and hover states.
  pub fn with_icon_src(mut self, src: &'static str) -> Self {
    self.reset.src = src;
    self.hover.src = src;
    self
  }

  /// Sets the Tailwind class on the reset state only.
  pub fn with_icon_class(mut self, class: &'static str) -> Self {
    self.reset.class = Some(class);
    self
  }

  /// Sets the inline style on the reset state only.
  pub fn with_icon_style(mut self, style: &'static str) -> Self {
    self.reset.style = Some(style);
    self
  }

  /// Overrides the asset path on the hover state only.
  pub fn with_icon_hover_src(mut self, src: &'static str) -> Self {
    self.hover.src = src;
    self
  }

  /// Sets the Tailwind class on the hover state only.
  pub fn with_icon_hover_class(mut self, class: &'static str) -> Self {
    self.hover.class = Some(class);
    self
  }

  /// Sets the inline style on the hover state only.
  pub fn with_icon_hover_style(mut self, style: &'static str) -> Self {
    self.hover.style = Some(style);
    self
  }
}
