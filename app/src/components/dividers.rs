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
//! - **Dark mode support** - Automatic theme adaptation
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
//!     <h2>"Section 2"</h2>
//!
//!     // Line with purple dot
//!     <Divider config=Divider::default_with_dot() />
//!
//!     <h3>"Section 3"</h3>
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
//! // Divider with default purple dot
//! <Divider config=Divider::default_with_dot() />
//! ```
//!
//! ### Custom Dot Colors
//!
//! ```rust
//! // Blue dot with 80% opacity
//! <Divider config=Divider::default()
//!   .with_dot("bg-blue-500/80")
//! />
//!
//! // Emerald dot with 90% opacity
//! <Divider config=Divider::default()
//!   .with_dot("bg-emerald-400/90")
//! />
//!
//! // Orange dot with default opacity
//! <Divider config=Divider::default()
//!   .with_dot("bg-orange-500/70")
//! />
//! ```
//!
//! ### Spacing Control
//!
//! ```rust
//! // Symmetric gaps around dot (6 = 24px on each side)
//! <Divider config=Divider::default()
//!   .with_dot("bg-purple-400/70")
//!   .with_dot_pos(6, 6)
//! />
//!
//! // Asymmetric gaps (tight left, loose right)
//! <Divider config=Divider::default()
//!   .with_dot("bg-purple-400/70")
//!   .with_dot_pos(4, 8)
//! />
//!
//! // Custom padding
//! <Divider config=Divider::default()
//!   .with_padding("py-6")
//! />
//! ```
//!
//! ### Container Width
//!
//! ```rust
//! // Narrow centered container
//! <Divider config=Divider::default()
//!   .with_dot("bg-purple-400/70")
//!   .with_margin("mx-auto max-w-2xl")
//! />
//!
//! // Wide container with fixed margins
//! <Divider config=Divider::default()
//!   .with_margin("mx-24 max-w-5xl")
//! />
//!
//! // Full width
//! <Divider config=Divider::default()
//!   .with_margin("w-full")
//! />
//! ```
//!
//! ### Complete Example
//!
//! ```rust
//! #[component]
//! pub fn Footer() -> impl IntoView {
//!   view! {
//!     <footer>
//!       // Plain separator above content
//!       <Divider />
//!
//!       <nav class="grid gap-4">
//!         <Socials />
//!         <Facets />
//!       </nav>
//!
//!       // Decorative dot separator
//!       <Divider config=Divider::default()
//!         .with_dot("bg-purple-400/70")
//!         .with_margin("mx-auto max-w-3xl")
//!       />
//!
//!       <Copyright />
//!     </footer>
//!   }
//! }
//! ```
//!
//! ## Legacy Components
//!
//! For backward compatibility, the original `DividerHr` and `DividerHrDot` components
//! are still available but the new `Divider` component is recommended.
//!
//! ```rust
//! // Legacy simple divider
//! <DividerHr />
//!
//! // Legacy dot divider with custom color
//! <DividerHrDot dot_color=Some("bg-pink-500/80") />
//! ```

use crate::prelude::*;

/// Configuration for a horizontal divider with optional centered dot accent.
///
/// Use the builder methods to customize appearance and spacing. The divider
/// features tapered gradient lines that fade in from the left and fade out
/// to the right, creating an elegant visual separation effect.
///
/// # Examples
///
/// ```rust
/// // Plain divider
/// let plain = Divider::default();
///
/// // With purple dot (default style)
/// let with_dot = Divider::default_with_dot();
///
/// // Custom blue dot with asymmetric gaps
/// let custom = Divider::default()
///   .with_dot("bg-blue-500/80")
///   .with_dot_pos(4, 8)
///   .with_margin("mx-auto max-w-4xl");
/// ```
#[derive(Clone, Copy)]
pub struct Divider {
  /// Base line color (e.g., "slate-300")
  pub line_color: &'static str,

  /// Line opacity weight (not typically modified)
  pub line_weight: &'static str,

  /// Complete Tailwind class for dot (e.g., "bg-purple-400/70")
  /// Pass the full class string so Tailwind JIT can detect it
  pub dot_class: Option<&'static str>,

  /// Gap spacing (left, right) in Tailwind units
  /// e.g., (6, 6) creates w-6 gaps = 24px on each side
  pub dot_pos: Option<(u8, u8)>,

  /// Container padding (e.g., "py-4")
  pub padding: Option<&'static str>,

  /// Container margin/width constraints (e.g., "mx-auto max-w-2xl")
  pub margin: Option<&'static str>,
}

impl Default for Divider {
  fn default() -> Self {
    Self {
      line_color: "slate-300",
      line_weight: "10",
      dot_class: None,
      dot_pos: None,
      padding: Some("py-4"),
      margin: None,
    }
  }
}

impl Divider {
  /// Creates a divider with the default teal dot accent and centered layout.
  ///
  /// The divider is centered horizontally with a moderate width constraint,
  /// creating a balanced visual separation that's narrower than full-width
  /// content but still prominent. Features comfortable 16px spacing on each
  /// side of the dot.
  ///
  /// Equivalent to:
  /// ```rust
  /// Divider::default()
  ///   .with_dot("bg-teal-400/70")
  ///   .with_dot_pos(4, 4)
  ///   .with_padding("py-4")
  ///   .with_margin("mx-auto max-w-xl")
  /// ```
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Use default centered dot divider
  /// <Divider config=Divider::default_with_dot() />
  ///
  /// // Tighter spacing around dot
  /// <Divider config=Divider::default_with_dot()
  ///   .with_dot_pos(3, 3)
  /// />
  ///
  /// // Wider container
  /// <Divider config=Divider::default_with_dot()
  ///   .with_margin("mx-auto max-w-2xl")
  /// />
  ///
  /// // Different color
  /// <Divider config=Divider::default_with_dot()
  ///   .with_dot("bg-purple-400/70")
  /// />
  /// ```
  pub fn default_with_dot() -> Self {
    Self::default()
      .with_dot(PRIMARY_ACCENT)
      .with_dot_pos(4, 4)
      .with_padding("py-4")
      .with_margin("mx-auto max-w-xl")
  }

  /// Adds a centered dot with a custom Tailwind background class.
  ///
  /// Automatically sets symmetric gaps of 6 units (24px) if not already configured.
  /// Use full Tailwind class strings (e.g., "bg-blue-500/80") so the JIT compiler
  /// can detect and generate the CSS.
  ///
  /// # Arguments
  ///
  /// * `class` - Complete Tailwind background class (e.g., "bg-purple-400/70")
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Purple dot with 70% opacity
  /// .with_dot("bg-purple-400/70")
  ///
  /// // Blue dot with 80% opacity
  /// .with_dot("bg-blue-500/80")
  ///
  /// // Emerald dot with 90% opacity
  /// .with_dot("bg-emerald-400/90")
  /// ```
  pub fn with_dot(mut self, class: &'static str) -> Self {
    self.dot_class = Some(class);
    if self.dot_pos.is_none() {
      self.dot_pos = Some((6, 6));
    }
    self
  }

  /// Sets the gap spacing on each side of the dot.
  ///
  /// Values correspond to Tailwind spacing units (4px per unit):
  /// - 4 = 16px (tight)
  /// - 6 = 24px (default, comfortable)
  /// - 8 = 32px (loose)
  ///
  /// # Arguments
  ///
  /// * `left` - Gap before the dot (Tailwind units)
  /// * `right` - Gap after the dot (Tailwind units)
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Symmetric gaps (24px each side)
  /// .with_dot_pos(6, 6)
  ///
  /// // Asymmetric (16px left, 32px right)
  /// .with_dot_pos(4, 8)
  ///
  /// // Tight spacing (8px each side)
  /// .with_dot_pos(2, 2)
  /// ```
  pub fn with_dot_pos(mut self, left: u8, right: u8) -> Self {
    self.dot_pos = Some((left, right));
    self
  }

  /// Sets vertical padding for the divider container.
  ///
  /// # Arguments
  ///
  /// * `padding` - Tailwind padding class (e.g., "py-3", "py-6")
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Compact spacing
  /// .with_padding("py-2")
  ///
  /// // Default spacing
  /// .with_padding("py-4")
  ///
  /// // Loose spacing
  /// .with_padding("py-8")
  /// ```
  pub fn with_padding(mut self, padding: &'static str) -> Self {
    self.padding = Some(padding);
    self
  }

  /// Sets horizontal margin and max-width constraints.
  ///
  /// Controls the overall width and centering of the divider.
  ///
  /// # Arguments
  ///
  /// * `margin` - Tailwind margin/width classes
  ///
  /// # Examples
  ///
  /// ```rust
  /// // Narrow centered
  /// .with_margin("mx-auto max-w-2xl")
  ///
  /// // Wide with fixed margins
  /// .with_margin("mx-24 max-w-5xl")
  ///
  /// // Full width
  /// .with_margin("w-full")
  ///
  /// // Responsive
  /// .with_margin("mx-4 md:mx-12 lg:mx-24 max-w-6xl")
  /// ```
  pub fn with_margin(mut self, margin: &'static str) -> Self {
    self.margin = Some(margin);
    self
  }

  /// Generates the Tailwind class for the left line (fades in from transparent).
  ///
  /// Internal method used by the component to create gradient effects.
  pub fn line_class_left(&self) -> String {
    format!(
      "bg-[linear-gradient(to_right,transparent_0%,theme(colors.{}.300)_50%)] dark:bg-[linear-gradient(to_right,transparent_0%,theme(colors.slate.700)_50%)]",
      self
        .line_color
        .replace("-300", "")
        .replace("-400", "")
        .replace("-500", "")
    )
  }

  /// Generates the Tailwind class for the right line (fades out to transparent).
  ///
  /// Internal method used by the component to create gradient effects.
  pub fn line_class_right(&self) -> String {
    format!(
      "bg-[linear-gradient(to_right,theme(colors.{}.300)_50%,transparent_100%)] dark:bg-[linear-gradient(to_right,theme(colors.slate.700)_50%,transparent_100%)]",
      self
        .line_color
        .replace("-300", "")
        .replace("-400", "")
        .replace("-500", "")
    )
  }

  /// Returns whether this divider has a dot accent.
  pub fn has_dot(&self) -> bool {
    self.dot_class.is_some()
  }
}

/// Flexible horizontal divider component with optional centered dot.
///
/// Creates a tapered horizontal line that fades in from transparent on the left
/// and fades out to transparent on the right. Optionally displays a centered
/// decorative dot with configurable spacing.
///
/// # Props
///
/// * `config` - Configuration struct (optional, uses defaults if omitted)
///
/// # Examples
///
/// ```rust
/// // Plain divider with defaults
/// <Divider />
///
/// // With purple dot
/// <Divider config=Divider::default_with_dot() />
///
/// // Custom configuration
/// <Divider config=Divider::default()
///   .with_dot("bg-blue-500/80")
///   .with_dot_pos(4, 8)
///   .with_padding("py-6")
///   .with_margin("mx-auto max-w-3xl")
/// />
/// ```
#[component]
pub fn Divider(
  /// Divider configuration (optional, uses defaults if not provided)
  #[prop(default = Divider::default())]
  config: Divider,
) -> impl IntoView {
  let padding_class = config.padding.unwrap_or("py-3");
  let margin_class = config.margin.unwrap_or("");

  let (left_gap, right_gap) = if config.has_dot() {
    config.dot_pos.unwrap_or((6, 6))
  } else {
    (0, 0)
  };

  let left_spacer = if left_gap > 0 {
    format!("w-{}", left_gap)
  } else {
    String::new()
  };

  let right_spacer = if right_gap > 0 {
    format!("w-{}", right_gap)
  } else {
    String::new()
  };

  view! {
    <div class=format!("flex items-center {} {}", padding_class, margin_class)>

      // Left line (fade in from left)
      <div class=format!("flex-1 h-px {}", config.line_class_left()) />

      // Left spacer (gap before dot)
      {(!left_spacer.is_empty())
        .then(|| {
          view! { <div class=format!("{} flex-shrink-0", left_spacer) /> }
        })}

      // Optional centered dot
      {config
        .has_dot()
        .then(|| {
          view! {
            <span class=format!(
              "w-1.5 h-1.5 rounded-full shrink-0 {}",
              config.dot_class.unwrap(),
            ) />
          }
        })}

      // Right spacer (gap after dot)
      {(!right_spacer.is_empty())
        .then(|| {
          view! { <div class=format!("{} flex-shrink-0", right_spacer) /> }
        })}

      // Right line (fade out to right)
      <div class=format!("flex-1 h-px {}", config.line_class_right()) />

    </div>
  }
}

/// Legacy simple horizontal divider (deprecated).
///
/// Creates a full-width horizontal line that tapers at both ends.
/// Consider using the new `Divider` component instead for more flexibility.
///
/// # Examples
///
/// ```rust
/// <DividerHr />
/// ```
#[component]
pub fn DividerHr() -> impl IntoView {
  view! {
    <div class="flex items-center py-3">
      <div class="w-full h-px bg-[linear-gradient(to_right,transparent,var(--color-slate-300),transparent)] dark:bg-[linear-gradient(to_right,transparent,var(--color-slate-700),transparent)]" />
    </div>
  }
}

/// Legacy horizontal divider with centered dot (deprecated).
///
/// Creates a tapered line with a centered dot accent. Fixed layout with
/// mx-36 max-w-4xl constraints. Consider using the new `Divider` component
/// for more flexibility.
///
/// # Props
///
/// * `dot_color` - Optional Tailwind background class (defaults to purple)
///
/// # Examples
///
/// ```rust
/// // Default purple dot
/// <DividerHrDot dot_color=None />
///
/// // Custom blue dot
/// <DividerHrDot dot_color=Some("bg-blue-500/80") />
/// ```
#[component]
pub fn DividerHrDot(
  /// Optional Tailwind color class for the dot (e.g., "bg-blue-500/80")
  dot_color: Option<&'static str>,
) -> impl IntoView {
  let dot_color = dot_color.unwrap_or("bg-purple-400/70");

  view! {
    <div class="flex gap-6 items-center py-3 mx-36 max-w-4xl">
      // Left line fade in
      <div class="flex-1 h-px bg-[linear-gradient(to_right,transparent_0%,var(--color-slate-300)_50%)] dark:bg-[linear-gradient(to_right,transparent_0%,var(--color-slate-700)_50%)]" />

      // Dot
      <span class=format!("w-1.5 h-1.5 rounded-full shrink-0 {}", dot_color) />

      // Right line fade out
      <div class="flex-1 h-px bg-[linear-gradient(to_right,var(--color-slate-300)_50%,transparent_100%)] dark:bg-[linear-gradient(to_right,var(--color-slate-700)_50%,transparent_100%)]" />
    </div>
  }
}
