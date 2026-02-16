//! # Divider Components
//!
//! Flexible horizontal divider components with optional centered dots and tapered line effects.
//!
//! ## Features
//!
//! - **Tapered lines** - Gradients fade in/out at edges for elegant visual separation
//! - **Optional dot accent** - Centered decorative dot with configurable colors
//! - **Flexible spacing** - Customizable gaps, padding, and margins
//! - **Builder pattern** - Chainable methods for clean configuration
//! - **Dark mode support** - Automatic theme adaptation via CSS variables
//!
//! ## Quick Start
//!
//! ```rust
//! use crate::prelude::*;
//!
//! #[component]
//! pub fn MyPage() -> impl IntoView {
//!   view! {
//!     <h1>"Section 1"</h1>
//!
//!     // Simple horizontal line
//!     <Divider />
//!
//!     // Line with default teal dot
//!     <Divider config=Divider::default_with_dot() />
//!
//!     // Line with secondary (purple) dot
//!     <Divider config=Divider::default_with_dot()
//!       .with_dot(SECONDARY_ACCENT)
//!     />
//!   }
//! }
//! ```
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! // Plain divider with defaults
//! <Divider />
//!
//! // Divider with default teal dot
//! <Divider config=Divider::default_with_dot() />
//! ```
//!
//! ### Custom Dot Colors
//!
//! ```rust
//! use crate::components::colors::*;
//!
//! // Primary (teal) dot
//! <Divider config=Divider::default().with_dot(PRIMARY_ACCENT) />
//!
//! // Secondary (purple) dot
//! <Divider config=Divider::default().with_dot(SECONDARY_ACCENT) />
//!
//! // Neutral dot
//! <Divider config=Divider::default().with_dot(NEUTRAL_ACCENT) />
//! ```
//!
//! ### Spacing Control
//!
//! ```rust
//! // Symmetric gaps around dot (6 = 24px on each side)
//! <Divider config=Divider::default()
//!   .with_dot(PRIMARY_ACCENT)
//!   .with_dot_pos(6, 6)
//! />
//!
//! // Asymmetric gaps (tight left, loose right)
//! <Divider config=Divider::default()
//!   .with_dot(PRIMARY_ACCENT)
//!   .with_dot_pos(4, 8)
//! />
//! ```
//!
//! ### Container Width
//!
//! ```rust
//! // Narrow centered container
//! <Divider config=Divider::default()
//!   .with_dot(PRIMARY_ACCENT)
//!   .with_margin("mx-auto max-w-2xl")
//! />
//!
//! // Full width
//! <Divider config=Divider::default().with_margin("w-full") />
//! ```
//!
//! ### Complete Example
//!
//! ```rust
//! #[component]
//! pub fn Footer() -> impl IntoView {
//!   view! {
//!     <footer>
//!       <Divider />
//!       <nav class="grid gap-4">
//!         <Socials />
//!         <Facets />
//!       </nav>
//!       <Divider config=Divider::default()
//!         .with_dot(SECONDARY_ACCENT)
//!         .with_margin("mx-auto max-w-3xl")
//!       />
//!       <Copyright />
//!     </footer>
//!   }
//! }
//! ```
//!
//! ## Migration from Legacy Components
//!
//! ```rust
//! // Before
//! <DividerHr />
//! <DividerHrDot dot_color=Some("bg-pink-500/80") />
//!
//! // After
//! <Divider />
//! <Divider config=Divider::default().with_dot("bg-pink-500/80") />
//! ```

use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Line Gradient Helpers                                     ║
//║                                                           ║
//║ Uses `var(--color-{name}-{step})` syntax                  ║
//║ The line color step is always 300 (light) / 700 (dark)    ║
//╚═══════════════════════════════════════════════════════════╝
/// Generates a left-side gradient class string: transparent → color at center.
///
/// `base` should be a plain Tailwind color name, e.g., `"slate"`.
fn line_gradient_left(base: &str) -> String {
  format!(
    "bg-[linear-gradient(to_right,transparent_0%,var(--color-{base}-300)_50%)] dark:bg-[linear-gradient(to_right,transparent_0%,var(--color-{base}-700)_50%)]"
  )
}

/// Generates a right-side gradient class string: color at center → transparent.
///
/// `base` should be a plain Tailwind color name, e.g., `"slate"`.
fn line_gradient_right(base: &str) -> String {
  format!(
    "bg-[linear-gradient(to_right,var(--color-{base}-300)_50%,transparent_100%)] dark:bg-[linear-gradient(to_right,var(--color-{base}-700)_50%,transparent_100%)]"
  )
}

//╔═══════════════════════════════════════════════════════════╗
//║ Configuration                                             ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a [`Divider`] component.
///
/// Built via the builder pattern — all methods consume and return `Self`.
/// Use [`Divider::default`] for a plain line or [`Divider::default_with_dot`]
/// for the standard teal-dot preset.
///
/// # Examples
///
/// ```rust
/// use app::{
///   components::Divider,
///   constants::colors::SECONDARY_ACCENT,
/// };
/// // Plain
/// Divider::default()
///
/// // Standard dot preset
/// Divider::default_with_dot()
///
/// // Custom
/// Divider::default()
///   .with_dot(SECONDARY_ACCENT)
///   .with_dot_pos(4, 8)
///   .with_margin("mx-auto max-w-4xl")
/// ```
#[derive(Clone, Copy)]
pub struct Divider {
  /// Base Tailwind color name for the line gradient (e.g., `"slate"`).
  /// Used to construct `var(--color-{line_base}-300)` / `var(--color-{line_base}-700)`.
  pub line_base: &'static str,

  /// Complete Tailwind class string for the dot (e.g., `"bg-teal-500/70 dark:bg-teal-400/70"`).
  /// Use constants from `colors.rs` — they are already theme-aware.
  pub dot_class: Option<&'static str>,

  /// Gap spacing (left, right) in Tailwind units (1 unit = 4px).
  pub dot_pos: Option<(u8, u8)>,

  /// Vertical padding class (e.g., `"py-4"`).
  pub padding: &'static str,

  /// Horizontal margin / width constraint (e.g., `"mx-auto max-w-xl"`).
  pub margin: &'static str,
}

impl Default for Divider {
  fn default() -> Self {
    Self {
      line_base: "slate",
      dot_class: None,
      dot_pos: None,
      padding: "py-2 md:py-6",
      margin: "",
    }
  }
}

impl Divider {
  /// Standard centered divider with the default primary (teal) dot accent.
  ///
  /// Equivalent to:
  /// ```rust
  /// use app::{
  ///   components::Divider,
  ///   constants::colors::PRIMARY_ACCENT,
  /// };
  /// Divider::default()
  ///   .with_dot(PRIMARY_ACCENT)
  ///   .with_dot_pos(4, 4)
  ///   .with_margin("mx-auto max-w-xl");
  /// ```
  pub fn default_with_dot() -> Self {
    Self::default()
      .with_dot(PRIMARY_ACCENT)
      .with_dot_pos(4, 4)
      .with_margin("mx-auto max-w-xl")
  }

  /// Adds a centered dot with the given Tailwind background class.
  ///
  /// Pass a theme-aware constant from `colors.rs` (e.g., `PRIMARY_ACCENT`, `SECONDARY_ACCENT`),
  /// or any raw Tailwind class string. Automatically sets symmetric 6-unit gaps if
  /// `with_dot_pos` has not already been called.
  pub fn with_dot(mut self, class: &'static str) -> Self {
    self.dot_class = Some(class);
    if self.dot_pos.is_none() {
      self.dot_pos = Some((6, 6));
    }
    self
  }

  /// Sets the gap on each side of the dot in Tailwind spacing units (1 unit = 4px).
  ///
  /// Common values: 2 = 8px, 4 = 16px, 6 = 24px (default), 8 = 32px.
  pub fn with_dot_pos(mut self, left: u8, right: u8) -> Self {
    self.dot_pos = Some((left, right));
    self
  }

  /// Overrides the vertical padding class (default: `"py-4"`).
  pub fn with_padding(mut self, padding: &'static str) -> Self {
    self.padding = padding;
    self
  }

  /// Sets the horizontal margin and/or max-width constraint.
  ///
  /// Examples: `"mx-auto max-w-2xl"`, `"mx-24"`, `"w-full"`.
  pub fn with_margin(mut self, margin: &'static str) -> Self {
    self.margin = margin;
    self
  }

  /// Overrides the line base color (default: `"slate"`).
  ///
  /// Must be a plain Tailwind color name (e.g., `"teal"`, `"zinc"`).
  pub fn with_line_color(mut self, base: &'static str) -> Self {
    self.line_base = base;
    self
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Component                                                 ║
//╚═══════════════════════════════════════════════════════════╝
/// Flexible horizontal divider with optional centered dot accent.
///
/// Creates tapered gradient lines that fade in from the left edge and fade out
/// at the right edge, with an optional decorative dot in the center.
///
/// # Props
///
/// * `config` - [`Divider`] (optional, uses defaults if omitted)
///
/// # Examples
///
/// ```rust
/// // Plain divider
/// <Divider />
///
/// // Standard dot preset
/// <Divider config=Divider::default_with_dot() />
///
/// // Custom
/// <Divider config=Divider::default()
///   .with_dot(SECONDARY_ACCENT)
///   .with_dot_pos(4, 8)
///   .with_margin("mx-auto max-w-3xl")
/// />
/// ```
#[component]
pub fn Divider(
  /// Divider configuration (optional — uses [`Divider::default`] if omitted)
  #[prop(default = Divider::default())]
  config: Divider,
) -> impl IntoView {
  let left_gradient = line_gradient_left(config.line_base);
  let right_gradient = line_gradient_right(config.line_base);

  let (left_gap, right_gap) = config.dot_pos.unwrap_or((0, 0));

  view! {
    <div class=format!("flex items-center {} {}", config.padding, config.margin)>

      // ~@ Left line — fades in from transparent
      <div class=format!("flex-1 h-px {left_gradient}") />

      // ~@ Left spacer
      {(left_gap > 0).then(|| view! { <div class=format!("w-{left_gap} flex-shrink-0") /> })}

      // ~@ Dot
      {config
        .dot_class
        .map(|cls| view! { <span class=format!("w-1.5 h-1.5 rounded-full shrink-0 {cls}") /> })}

      // ~@ Right spacer
      {(right_gap > 0).then(|| view! { <div class=format!("w-{right_gap} flex-shrink-0") /> })}

      // ~@ Right line — fades out to transparent
      <div class=format!("flex-1 h-px {right_gradient}") />

    </div>
  }
}

/// Legacy simple divider. Use [`Divider`] instead.
#[component]
pub fn DividerHr() -> impl IntoView {
  view! {
    <div class="flex items-center py-3">
      <div class="w-full h-px bg-[linear-gradient(to_right,transparent,var(--color-slate-300),transparent)] dark:bg-[linear-gradient(to_right,transparent,var(--color-slate-700),transparent)]" />
    </div>
  }
}

/// Legacy dot divider. Use [`Divider`] with `.with_dot()` instead.
#[component]
pub fn DividerHrDot(
  /// Optional Tailwind background class for the dot (e.g., `"bg-blue-500/80"`).
  /// Defaults to the primary accent if `None`.
  dot_color: Option<&'static str>,
) -> impl IntoView {
  let dot_color = dot_color.unwrap_or(PRIMARY_ACCENT);

  view! {
    <div class="flex gap-6 items-center py-3 mx-36 max-w-4xl">
      <div class="flex-1 h-px bg-[linear-gradient(to_right,transparent_0%,var(--color-slate-300)_50%)] dark:bg-[linear-gradient(to_right,transparent_0%,var(--color-slate-700)_50%)]" />
      <span class=format!("w-1.5 h-1.5 rounded-full shrink-0 {dot_color}") />
      <div class="flex-1 h-px bg-[linear-gradient(to_right,var(--color-slate-300)_50%,transparent_100%)] dark:bg-[linear-gradient(to_right,var(--color-slate-700)_50%,transparent_100%)]" />
    </div>
  }
}
