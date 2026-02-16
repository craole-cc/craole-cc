//! # Color System
//!
//! Centralized, theme-aware color definitions for consistent styling across the application.
//! All constants are complete Tailwind CSS class strings, automatically light/dark mode aware.
//!
//! ## Design
//!
//! Colors follow a **numeric scale (100–900)** mirroring Tailwind's own palette conventions:
//! - **100–300**: Light tints (backgrounds, subtle fills)
//! - **400–600**: Mid-range (borders, accents, interactive states)
//! - **700–900**: Dark shades (text, strong emphasis)
//!
//! The light/dark split within each constant is intentional:
//! - Light mode uses the lower end of the scale (e.g., `teal-100` bg, `teal-700` text)
//! - Dark mode inverts: darker backgrounds, lighter text and accents
//!
//! ## Changing a palette color
//!
//! Edit the base name in the `define_palette!` calls — every constant and `Palette`
//! method updates automatically. No find-and-replace needed.
//!
//! ```
//! define_palette!(PRIMARY,   "teal");    // change "teal" → "cyan" to retheme
//! define_palette!(SECONDARY, "purple");
//! define_palette!(NEUTRAL,   "slate");
//! ```
//!
//! ## Usage
//!
//! Constants are plain `&'static str` — pass them wherever a class string is expected:
//!
//! ```rust
//! use app::constants::colors::*;
//!
//! // Static constants produce complete, theme-aware Tailwind class strings
//! assert!(PRIMARY_BG_200.contains("teal"));
//! assert!(PRIMARY_BG_200.contains("dark:"));
//!
//! // Semantic aliases resolve to a specific scale step
//! assert_eq!(PRIMARY_BG,   PRIMARY_BG_100);
//! assert_eq!(PRIMARY_TEXT, PRIMARY_TEXT_700);
//!
//! // Compose with other classes using format!
//! let card = format!("{} rounded-lg p-4", PRIMARY_BG_200);
//! assert!(card.starts_with("bg-teal"));
//!
//! // Palette selects a variant at runtime
//! assert_eq!(Palette::Secondary.bg(200), SECONDARY_BG_200);
//! assert_eq!(Palette::Neutral.border(300), NEUTRAL_BORDER_300);
//!
//! // In a Leptos component you would write:
//! // <div class={format!("{} rounded-lg", PRIMARY_BG_200)} />
//! // <Divider config=DividerConfig::default().with_dot(Palette::Secondary.accent(500)) />
//! ```
//!
//! ## Tailwind Config
//!
//! Do **not** define semantic tokens (bg-primary, text-secondary, etc.) in tailwind.config.js.
//! Those bypass Tailwind's built-in dark mode variant and lose opacity modifier support.
//! Only add to the Tailwind config for genuinely custom hex colors absent from Tailwind's palette.

use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Palette Macro                                             ║
//╚═══════════════════════════════════════════════════════════╝
/// Generates all theme-aware constants for a named palette from a single Tailwind color base.
///
/// The base name (e.g., `"teal"`) is the single source of truth — change it here
/// and every constant, alias, and `Palette` method updates automatically.
///
/// Generates constants in the pattern `{PREFIX}_{CATEGORY}_{STEP}` covering:
/// - Backgrounds: 100, 200, 300
/// - Accents (semi-transparent): 400, 500, 600
/// - Solid backgrounds: 400, 500, 600
/// - Text: 600, 700, 800
/// - Borders: 300, 400, 500
/// - Rings: 400, 500
/// - Hover states: 100, 200
/// - Semantic aliases: BG, TEXT, ACCENT, BG_SOLID, BORDER, RING, HOVER
macro_rules! define_palette {
  ($prefix:ident, $base:literal) => {
    paste! {
      //~@ Backgrounds (subtle → strong)
      /// Lightest background — hero sections, page-level wash
      pub const [<$prefix _BG_100>]: &str =
        concat!("bg-", $base, "-100 dark:bg-", $base, "-950");
      /// Soft background — cards, panels, hover targets
      pub const [<$prefix _BG_200>]: &str =
        concat!("bg-", $base, "-200 dark:bg-", $base, "-900");
      /// Muted background — active selections, chips
      pub const [<$prefix _BG_300>]: &str =
        concat!("bg-", $base, "-300 dark:bg-", $base, "-800");

      //~@ Accents (semi-transparent overlays, badges, indicators)
      /// Light accent overlay — subtle highlights
      pub const [<$prefix _ACCENT_400>]: &str =
        concat!("bg-", $base, "-400/50 dark:bg-", $base, "-500/50");
      /// Standard accent overlay — badges, dots, pills
      pub const [<$prefix _ACCENT_500>]: &str =
        concat!("bg-", $base, "-500/70 dark:bg-", $base, "-400/70");
      /// Strong accent overlay — notifications, emphasis
      pub const [<$prefix _ACCENT_600>]: &str =
        concat!("bg-", $base, "-600/90 dark:bg-", $base, "-400/90");

      //~@ Solid backgrounds (opaque fills for buttons, tags, etc.)
      /// Light solid — secondary buttons, tags
      pub const [<$prefix _BG_SOLID_400>]: &str =
        concat!("bg-", $base, "-400 dark:bg-", $base, "-600");
      /// Standard solid — primary buttons, CTAs
      pub const [<$prefix _BG_SOLID_500>]: &str =
        concat!("bg-", $base, "-500 dark:bg-", $base, "-500");
      /// Dark solid — pressed states, emphasis
      pub const [<$prefix _BG_SOLID_600>]: &str =
        concat!("bg-", $base, "-600 dark:bg-", $base, "-400");

      //~@ Text
      /// Light body text — captions, secondary labels
      pub const [<$prefix _TEXT_600>]: &str =
        concat!("text-", $base, "-600 dark:text-", $base, "-400");
      /// Standard body text — default colored text
      pub const [<$prefix _TEXT_700>]: &str =
        concat!("text-", $base, "-700 dark:text-", $base, "-300");
      /// Strong text — headings, emphasis
      pub const [<$prefix _TEXT_800>]: &str =
        concat!("text-", $base, "-800 dark:text-", $base, "-200");

      //~@ Borders
      /// Subtle border — dividers, input outlines
      pub const [<$prefix _BORDER_300>]: &str =
        concat!("border-", $base, "-300 dark:border-", $base, "-700");
      /// Standard border — cards, sections
      pub const [<$prefix _BORDER_400>]: &str =
        concat!("border-", $base, "-400 dark:border-", $base, "-600");
      /// Strong border — active inputs, selected states
      pub const [<$prefix _BORDER_500>]: &str =
        concat!("border-", $base, "-500 dark:border-", $base, "-500");

      //~@ Rings (focus indicators)
      /// Standard focus ring
      pub const [<$prefix _RING_400>]: &str =
        concat!("ring-", $base, "-400 dark:ring-", $base, "-500");
      /// Strong focus ring — accessibility emphasis
      pub const [<$prefix _RING_500>]: &str =
        concat!("ring-", $base, "-500 dark:ring-", $base, "-400");

      //~@ Hover states
      /// Subtle hover — list items, nav links
      pub const [<$prefix _HOVER_100>]: &str =
        concat!("hover:bg-", $base, "-100 dark:hover:bg-", $base, "-900/40");
      /// Standard hover — buttons, interactive tiles
      pub const [<$prefix _HOVER_200>]: &str =
        concat!("hover:bg-", $base, "-200 dark:hover:bg-", $base, "-800/50");

      //~@ Semantic aliases (map intent → scale; change here to update all callsites)
      pub const [<$prefix _BG>]:       &str = [<$prefix _BG_100>];
      pub const [<$prefix _TEXT>]:     &str = [<$prefix _TEXT_700>];
      pub const [<$prefix _ACCENT>]:   &str = [<$prefix _ACCENT_500>];
      pub const [<$prefix _BG_SOLID>]: &str = [<$prefix _BG_SOLID_500>];
      pub const [<$prefix _BORDER>]:   &str = [<$prefix _BORDER_400>];
      pub const [<$prefix _RING>]:     &str = [<$prefix _RING_500>];
      pub const [<$prefix _HOVER>]:    &str = [<$prefix _HOVER_100>];
    }
  };
}

//╔═══════════════════════════════════════════════════════════╗
//║ Palette Definitions                                       ║
//║                                                           ║
//║ Single source of truth for brand colors.                  ║
//╚═══════════════════════════════════════════════════════════╝
define_palette!(PRIMARY, "teal");
define_palette!(SECONDARY, "purple");
define_palette!(NEUTRAL, "slate");

//╔═══════════════════════════════════════════════════════════╗
//║ SVG Filter Styles                                         ║
//╚═══════════════════════════════════════════════════════════╝
/// Renders a hard-black SVG as mid-grey.
///
/// `brightness(0)` collapses all pixels to black, then `invert(0.35)` lifts
/// them to ~35% grey. Adjust the invert value to taste.
///
/// Use for monochrome icons (e.g. GitHub) that ship as black SVGs.
///
/// ```rust
/// use app::constants::colors::GREY_FROM_BLACK;
/// assert!(GREY_FROM_BLACK.contains("brightness(0)"));
/// assert!(GREY_FROM_BLACK.contains("invert(0.35)"));
/// ```
pub const GREY_FROM_BLACK: &str = "filter: brightness(0) invert(0.35);";

/// Dims a solid-colour SVG to match the grey tone of [`GREY_FROM_BLACK`]
/// without flattening the image to a solid block.
///
/// Use for branded colour icons (e.g. LinkedIn, Facebook) where you want
/// muted parity with greyscale icons.
///
/// ```rust
/// use app::constants::colors::DIM_COLOUR;
/// assert!(DIM_COLOUR.contains("opacity"));
/// ```
pub const DIM_COLOUR: &str = "opacity: 0.6;";

//╔═══════════════════════════════════════════════════════════╗
//║ Palette — runtime variant selection                       ║
//╚═══════════════════════════════════════════════════════════╝
/// Selects a brand palette at runtime and exposes its theme-aware class strings
/// via typed accessor methods.
///
/// All methods return `&'static str` — zero allocation, fully Tailwind-scannable.
///
/// # Examples
///
/// ```rust
/// use app::constants::colors::*;
///
/// // Each variant returns its own set of class strings
/// assert_eq!(Palette::Primary.bg(100),      PRIMARY_BG_100);
/// assert_eq!(Palette::Secondary.text(700),  SECONDARY_TEXT_700);
/// assert_eq!(Palette::Neutral.border(300),  NEUTRAL_BORDER_300);
/// assert_eq!(Palette::Primary.accent(500),  PRIMARY_ACCENT_500);
/// assert_eq!(Palette::Secondary.ring(500),  SECONDARY_RING_500);
///
/// // Default variant is Primary
/// assert_eq!(Palette::default(), Palette::Primary);
///
/// // Copy semantics — store a variant and call methods on it
/// let p = Palette::Secondary;
/// assert_eq!(p.bg(200),     SECONDARY_BG_200);
/// assert_eq!(p.hover(100),  SECONDARY_HOVER_100);
///
/// // In a Leptos component you would write:
/// // <div class={p.bg(200)} />
/// // <Divider config=DividerConfig::default().with_dot(p.accent(500)) />
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Palette {
  #[default]
  Primary,
  Secondary,
  Neutral,
}

impl Palette {
  /// Background fill — step: 100, 200, 300
  pub fn bg(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 100) => PRIMARY_BG_100,
      (Self::Primary, 200) => PRIMARY_BG_200,
      (Self::Primary, 300) => PRIMARY_BG_300,
      (Self::Secondary, 100) => SECONDARY_BG_100,
      (Self::Secondary, 200) => SECONDARY_BG_200,
      (Self::Secondary, 300) => SECONDARY_BG_300,
      (Self::Neutral, 100) => NEUTRAL_BG_100,
      (Self::Neutral, 200) => NEUTRAL_BG_200,
      (Self::Neutral, 300) => NEUTRAL_BG_300,
      _ => panic!("bg() step must be 100, 200, or 300"),
    }
  }

  /// Solid (opaque) background — step: 400, 500, 600
  pub fn bg_solid(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 400) => PRIMARY_BG_SOLID_400,
      (Self::Primary, 500) => PRIMARY_BG_SOLID_500,
      (Self::Primary, 600) => PRIMARY_BG_SOLID_600,
      (Self::Secondary, 400) => SECONDARY_BG_SOLID_400,
      (Self::Secondary, 500) => SECONDARY_BG_SOLID_500,
      (Self::Secondary, 600) => SECONDARY_BG_SOLID_600,
      (Self::Neutral, 400) => NEUTRAL_BG_SOLID_400,
      (Self::Neutral, 500) => NEUTRAL_BG_SOLID_500,
      (Self::Neutral, 600) => NEUTRAL_BG_SOLID_600,
      _ => panic!("bg_solid() step must be 400, 500, or 600"),
    }
  }

  /// Semi-transparent accent overlay — step: 400, 500, 600
  pub fn accent(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 400) => PRIMARY_ACCENT_400,
      (Self::Primary, 500) => PRIMARY_ACCENT_500,
      (Self::Primary, 600) => PRIMARY_ACCENT_600,
      (Self::Secondary, 400) => SECONDARY_ACCENT_400,
      (Self::Secondary, 500) => SECONDARY_ACCENT_500,
      (Self::Secondary, 600) => SECONDARY_ACCENT_600,
      (Self::Neutral, 400) => NEUTRAL_ACCENT_400,
      (Self::Neutral, 500) => NEUTRAL_ACCENT_500,
      (Self::Neutral, 600) => NEUTRAL_ACCENT_600,
      _ => panic!("accent() step must be 400, 500, or 600"),
    }
  }

  /// Text color — step: 600, 700, 800
  pub fn text(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 600) => PRIMARY_TEXT_600,
      (Self::Primary, 700) => PRIMARY_TEXT_700,
      (Self::Primary, 800) => PRIMARY_TEXT_800,
      (Self::Secondary, 600) => SECONDARY_TEXT_600,
      (Self::Secondary, 700) => SECONDARY_TEXT_700,
      (Self::Secondary, 800) => SECONDARY_TEXT_800,
      (Self::Neutral, 600) => NEUTRAL_TEXT_600,
      (Self::Neutral, 700) => NEUTRAL_TEXT_700,
      (Self::Neutral, 800) => NEUTRAL_TEXT_800,
      _ => panic!("text() step must be 600, 700, or 800"),
    }
  }

  /// Border color — step: 300, 400, 500
  pub fn border(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 300) => PRIMARY_BORDER_300,
      (Self::Primary, 400) => PRIMARY_BORDER_400,
      (Self::Primary, 500) => PRIMARY_BORDER_500,
      (Self::Secondary, 300) => SECONDARY_BORDER_300,
      (Self::Secondary, 400) => SECONDARY_BORDER_400,
      (Self::Secondary, 500) => SECONDARY_BORDER_500,
      (Self::Neutral, 300) => NEUTRAL_BORDER_300,
      (Self::Neutral, 400) => NEUTRAL_BORDER_400,
      (Self::Neutral, 500) => NEUTRAL_BORDER_500,
      _ => panic!("border() step must be 300, 400, or 500"),
    }
  }

  /// Ring (focus indicator) — step: 400, 500
  pub fn ring(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 400) => PRIMARY_RING_400,
      (Self::Primary, 500) => PRIMARY_RING_500,
      (Self::Secondary, 400) => SECONDARY_RING_400,
      (Self::Secondary, 500) => SECONDARY_RING_500,
      (Self::Neutral, 400) => NEUTRAL_RING_400,
      (Self::Neutral, 500) => NEUTRAL_RING_500,
      _ => panic!("ring() step must be 400 or 500"),
    }
  }

  /// Hover background — step: 100, 200
  pub fn hover(self, step: u16) -> &'static str {
    match (self, step) {
      (Self::Primary, 100) => PRIMARY_HOVER_100,
      (Self::Primary, 200) => PRIMARY_HOVER_200,
      (Self::Secondary, 100) => SECONDARY_HOVER_100,
      (Self::Secondary, 200) => SECONDARY_HOVER_200,
      (Self::Neutral, 100) => NEUTRAL_HOVER_100,
      (Self::Neutral, 200) => NEUTRAL_HOVER_200,
      _ => panic!("hover() step must be 100 or 200"),
    }
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tests                                                     ║
//╚═══════════════════════════════════════════════════════════╝
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn semantic_aliases_match_scale_constants() {
    assert_eq!(PRIMARY_BG, PRIMARY_BG_100);
    assert_eq!(PRIMARY_TEXT, PRIMARY_TEXT_700);
    assert_eq!(PRIMARY_ACCENT, PRIMARY_ACCENT_500);
    assert_eq!(PRIMARY_BORDER, PRIMARY_BORDER_400);
    assert_eq!(PRIMARY_RING, PRIMARY_RING_500);
    assert_eq!(NEUTRAL_BG, NEUTRAL_BG_100);
    assert_eq!(NEUTRAL_BORDER, NEUTRAL_BORDER_400);
  }

  #[test]
  fn palette_resolves_correctly() {
    assert_eq!(Palette::Primary.bg(100), PRIMARY_BG_100);
    assert_eq!(Palette::Primary.text(700), PRIMARY_TEXT_700);
    assert_eq!(Palette::Primary.accent(500), PRIMARY_ACCENT_500);
    assert_eq!(Palette::Primary.border(400), PRIMARY_BORDER_400);
    assert_eq!(Palette::Primary.ring(500), PRIMARY_RING_500);
    assert_eq!(Palette::Secondary.bg(200), SECONDARY_BG_200);
    assert_eq!(Palette::Secondary.text(600), SECONDARY_TEXT_600);
    assert_eq!(Palette::Secondary.accent(400), SECONDARY_ACCENT_400);
    assert_eq!(Palette::Neutral.bg(100), NEUTRAL_BG_100);
    assert_eq!(Palette::Neutral.border(300), NEUTRAL_BORDER_300);
  }

  #[test]
  fn all_constants_contain_dark_variant() {
    for class in [
      PRIMARY_BG_100,
      PRIMARY_BG_200,
      PRIMARY_BG_300,
      PRIMARY_TEXT_600,
      PRIMARY_TEXT_700,
      PRIMARY_TEXT_800,
      PRIMARY_BORDER_300,
      PRIMARY_BORDER_400,
      PRIMARY_BORDER_500,
      SECONDARY_BG_100,
      SECONDARY_TEXT_700,
      NEUTRAL_BG_100,
      NEUTRAL_BORDER_300,
      NEUTRAL_TEXT_700,
    ] {
      assert!(class.contains("dark:"), "Missing dark: variant in: {class}");
    }
  }

  #[test]
  fn constants_contain_expected_base_color() {
    assert!(PRIMARY_BG_100.contains("teal"), "PRIMARY should use teal");
    assert!(
      SECONDARY_BG_100.contains("purple"),
      "SECONDARY should use purple"
    );
    assert!(NEUTRAL_BG_100.contains("slate"), "NEUTRAL should use slate");
  }

  #[test]
  fn palette_default_is_primary() {
    assert_eq!(Palette::default(), Palette::Primary);
  }
}
