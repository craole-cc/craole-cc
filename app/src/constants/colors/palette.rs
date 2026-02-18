use crate::prelude::paste;

//╔═══════════════════════════════════════════════════════════╗
//║ Macros                                                    ║
//╚═══════════════════════════════════════════════════════════╝
/// Generates all theme-aware constants for a named palette from a single Tailwind color base.
///
/// The base name (e.g., `"teal"`) is the single source of truth — change it here
/// and every constant, alias, and `Palette` method updates automatically.
macro_rules! define_palette {
  ($prefix:ident, $base:literal) => {
    paste! {
      //~@ Backgrounds (subtle → strong)
      pub const [<$prefix _BG_100>]: &str = concat!("bg-", $base, "-100 dark:bg-", $base, "-950");
      pub const [<$prefix _BG_200>]: &str = concat!("bg-", $base, "-200 dark:bg-", $base, "-900");
      pub const [<$prefix _BG_300>]: &str = concat!("bg-", $base, "-300 dark:bg-", $base, "-800");

      //~@ Accents (semi-transparent overlays, badges, indicators)
      pub const [<$prefix _ACCENT_400>]: &str = concat!("bg-", $base, "-400/50 dark:bg-", $base, "-500/50");
      pub const [<$prefix _ACCENT_500>]: &str = concat!("bg-", $base, "-500/70 dark:bg-", $base, "-400/70");
      pub const [<$prefix _ACCENT_600>]: &str = concat!("bg-", $base, "-600/90 dark:bg-", $base, "-400/90");

      //~@ Solid backgrounds (opaque fills for buttons, tags, etc.)
      pub const [<$prefix _BG_SOLID_400>]: &str = concat!("bg-", $base, "-400 dark:bg-", $base, "-600");
      pub const [<$prefix _BG_SOLID_500>]: &str = concat!("bg-", $base, "-500 dark:bg-", $base, "-500");
      pub const [<$prefix _BG_SOLID_600>]: &str = concat!("bg-", $base, "-600 dark:bg-", $base, "-400");

      //~@ Text
      pub const [<$prefix _TEXT_600>]: &str = concat!("text-", $base, "-600 dark:text-", $base, "-400");
      pub const [<$prefix _TEXT_700>]: &str = concat!("text-", $base, "-700 dark:text-", $base, "-300");
      pub const [<$prefix _TEXT_800>]: &str = concat!("text-", $base, "-800 dark:text-", $base, "-200");

      //~@ Fill (for SVG icons)
      pub const [<$prefix _FILL_600>]: &str = concat!("fill-", $base, "-600 dark:fill-", $base, "-400");
      pub const [<$prefix _FILL_700>]: &str = concat!("fill-", $base, "-700 dark:fill-", $base, "-300");
      pub const [<$prefix _FILL_800>]: &str = concat!("fill-", $base, "-800 dark:fill-", $base, "-200");

      //~@ Borders
      pub const [<$prefix _BORDER_300>]: &str = concat!("border-", $base, "-300 dark:border-", $base, "-700");
      pub const [<$prefix _BORDER_400>]: &str = concat!("border-", $base, "-400 dark:border-", $base, "-600");
      pub const [<$prefix _BORDER_500>]: &str = concat!("border-", $base, "-500 dark:border-", $base, "-500");

      //~@ Rings (focus indicators)
      pub const [<$prefix _RING_400>]: &str = concat!("ring-", $base, "-400 dark:ring-", $base, "-500");
      pub const [<$prefix _RING_500>]: &str = concat!("ring-", $base, "-500 dark:ring-", $base, "-400");

      //~@ Hover states (backgrounds)
      pub const [<$prefix _HOVER_BG_100>]: &str = concat!("hover:bg-", $base, "-100 dark:hover:bg-", $base, "-900/40");
      pub const [<$prefix _HOVER_BG_200>]: &str = concat!("hover:bg-", $base, "-200 dark:hover:bg-", $base, "-800/50");

      //~@ Hover states (text colors)
      pub const [<$prefix _HOVER_TEXT_600>]: &str = concat!("hover:text-", $base, "-600 dark:hover:text-", $base, "-400");
      pub const [<$prefix _HOVER_TEXT_700>]: &str = concat!("hover:text-", $base, "-700 dark:hover:text-", $base, "-300");
      pub const [<$prefix _HOVER_TEXT_800>]: &str = concat!("hover:text-", $base, "-800 dark:hover:text-", $base, "-200");

      //~@ Surface backgrounds (component-level, not page-level fills)
      /// Solid card surface — white on light, very subtle tint on dark.
      pub const [<$prefix _BG_CARD>]:    &str = "bg-white dark:bg-white/5";
      /// Frosted card surface — translucent on light, near-invisible tint on dark.
      pub const [<$prefix _BG_SURFACE>]: &str = "bg-white/60 dark:bg-white/5";
      /// Ghost surface — transparent on light, barely-there tint on dark.
      pub const [<$prefix _BG_GHOST>]:   &str = "bg-transparent dark:bg-white/5";

      //~@ Semantic aliases (map intent → scale; change here to update all callsites)
      pub const [<$prefix _BG>]:         &str = [<$prefix _BG_100>];
      pub const [<$prefix _TEXT>]:       &str = [<$prefix _TEXT_700>];
      pub const [<$prefix _FILL>]:       &str = [<$prefix _FILL_800>];
      pub const [<$prefix _ACCENT>]:     &str = [<$prefix _ACCENT_500>];
      pub const [<$prefix _BG_SOLID>]:   &str = [<$prefix _BG_SOLID_500>];
      pub const [<$prefix _BORDER>]:     &str = [<$prefix _BORDER_400>];
      pub const [<$prefix _RING>]:       &str = [<$prefix _RING_500>];
      pub const [<$prefix _HOVER_BG>]:   &str = [<$prefix _HOVER_BG_100>];
      pub const [<$prefix _HOVER_TEXT>]: &str = [<$prefix _HOVER_TEXT_600>];
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
//║ Palette — Runtime Variant Selection                       ║
//╚═══════════════════════════════════════════════════════════╝
/// Selects a brand palette at runtime and exposes theme-aware class strings
/// via typed accessor methods. All methods return `&'static str`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq,)]
pub enum Palette {
  #[default]
  Primary,
  Secondary,
  Neutral,
}

impl Palette {
  /// Background fill — step: 100, 200, 300
  pub fn bg(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 100,) => PRIMARY_BG_100,
      | (Self::Primary, 200,) => PRIMARY_BG_200,
      | (Self::Primary, 300,) => PRIMARY_BG_300,
      | (Self::Secondary, 100,) => SECONDARY_BG_100,
      | (Self::Secondary, 200,) => SECONDARY_BG_200,
      | (Self::Secondary, 300,) => SECONDARY_BG_300,
      | (Self::Neutral, 100,) => NEUTRAL_BG_100,
      | (Self::Neutral, 200,) => NEUTRAL_BG_200,
      | (Self::Neutral, 300,) => NEUTRAL_BG_300,
      | _ => panic!("bg() step must be 100, 200, or 300"),
    }
  }

  /// Solid (opaque) background — step: 400, 500, 600
  pub fn bg_solid(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 400,) => PRIMARY_BG_SOLID_400,
      | (Self::Primary, 500,) => PRIMARY_BG_SOLID_500,
      | (Self::Primary, 600,) => PRIMARY_BG_SOLID_600,
      | (Self::Secondary, 400,) => SECONDARY_BG_SOLID_400,
      | (Self::Secondary, 500,) => SECONDARY_BG_SOLID_500,
      | (Self::Secondary, 600,) => SECONDARY_BG_SOLID_600,
      | (Self::Neutral, 400,) => NEUTRAL_BG_SOLID_400,
      | (Self::Neutral, 500,) => NEUTRAL_BG_SOLID_500,
      | (Self::Neutral, 600,) => NEUTRAL_BG_SOLID_600,
      | _ => panic!("bg_solid() step must be 400, 500, or 600"),
    }
  }

  /// Semi-transparent accent overlay — step: 400, 500, 600
  pub fn accent(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 400,) => PRIMARY_ACCENT_400,
      | (Self::Primary, 500,) => PRIMARY_ACCENT_500,
      | (Self::Primary, 600,) => PRIMARY_ACCENT_600,
      | (Self::Secondary, 400,) => SECONDARY_ACCENT_400,
      | (Self::Secondary, 500,) => SECONDARY_ACCENT_500,
      | (Self::Secondary, 600,) => SECONDARY_ACCENT_600,
      | (Self::Neutral, 400,) => NEUTRAL_ACCENT_400,
      | (Self::Neutral, 500,) => NEUTRAL_ACCENT_500,
      | (Self::Neutral, 600,) => NEUTRAL_ACCENT_600,
      | _ => panic!("accent() step must be 400, 500, or 600"),
    }
  }

  /// Text color — step: 600, 700, 800
  pub fn text(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 600,) => PRIMARY_TEXT_600,
      | (Self::Primary, 700,) => PRIMARY_TEXT_700,
      | (Self::Primary, 800,) => PRIMARY_TEXT_800,
      | (Self::Secondary, 600,) => SECONDARY_TEXT_600,
      | (Self::Secondary, 700,) => SECONDARY_TEXT_700,
      | (Self::Secondary, 800,) => SECONDARY_TEXT_800,
      | (Self::Neutral, 600,) => NEUTRAL_TEXT_600,
      | (Self::Neutral, 700,) => NEUTRAL_TEXT_700,
      | (Self::Neutral, 800,) => NEUTRAL_TEXT_800,
      | _ => panic!("text() step must be 600, 700, or 800"),
    }
  }

  /// Fill color (for SVG icons) — step: 600, 700, 800
  pub fn fill(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 600,) => PRIMARY_FILL_600,
      | (Self::Primary, 700,) => PRIMARY_FILL_700,
      | (Self::Primary, 800,) => PRIMARY_FILL_800,
      | (Self::Secondary, 600,) => SECONDARY_FILL_600,
      | (Self::Secondary, 700,) => SECONDARY_FILL_700,
      | (Self::Secondary, 800,) => SECONDARY_FILL_800,
      | (Self::Neutral, 600,) => NEUTRAL_FILL_600,
      | (Self::Neutral, 700,) => NEUTRAL_FILL_700,
      | (Self::Neutral, 800,) => NEUTRAL_FILL_800,
      | _ => panic!("fill() step must be 600, 700, or 800"),
    }
  }

  /// Border color — step: 300, 400, 500
  pub fn border(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 300,) => PRIMARY_BORDER_300,
      | (Self::Primary, 400,) => PRIMARY_BORDER_400,
      | (Self::Primary, 500,) => PRIMARY_BORDER_500,
      | (Self::Secondary, 300,) => SECONDARY_BORDER_300,
      | (Self::Secondary, 400,) => SECONDARY_BORDER_400,
      | (Self::Secondary, 500,) => SECONDARY_BORDER_500,
      | (Self::Neutral, 300,) => NEUTRAL_BORDER_300,
      | (Self::Neutral, 400,) => NEUTRAL_BORDER_400,
      | (Self::Neutral, 500,) => NEUTRAL_BORDER_500,
      | _ => panic!("border() step must be 300, 400, or 500"),
    }
  }

  /// Ring (focus indicator) — step: 400, 500
  pub fn ring(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 400,) => PRIMARY_RING_400,
      | (Self::Primary, 500,) => PRIMARY_RING_500,
      | (Self::Secondary, 400,) => SECONDARY_RING_400,
      | (Self::Secondary, 500,) => SECONDARY_RING_500,
      | (Self::Neutral, 400,) => NEUTRAL_RING_400,
      | (Self::Neutral, 500,) => NEUTRAL_RING_500,
      | _ => panic!("ring() step must be 400 or 500"),
    }
  }

  /// Hover background — step: 100, 200
  pub fn hover_bg(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 100,) => PRIMARY_HOVER_BG_100,
      | (Self::Primary, 200,) => PRIMARY_HOVER_BG_200,
      | (Self::Secondary, 100,) => SECONDARY_HOVER_BG_100,
      | (Self::Secondary, 200,) => SECONDARY_HOVER_BG_200,
      | (Self::Neutral, 100,) => NEUTRAL_HOVER_BG_100,
      | (Self::Neutral, 200,) => NEUTRAL_HOVER_BG_200,
      | _ => panic!("hover_bg() step must be 100 or 200"),
    }
  }

  /// Hover text color — step: 600, 700, 800
  pub fn hover_text(self, step : u16,) -> &'static str {
    match (self, step,) {
      | (Self::Primary, 600,) => PRIMARY_HOVER_TEXT_600,
      | (Self::Primary, 700,) => PRIMARY_HOVER_TEXT_700,
      | (Self::Primary, 800,) => PRIMARY_HOVER_TEXT_800,
      | (Self::Secondary, 600,) => SECONDARY_HOVER_TEXT_600,
      | (Self::Secondary, 700,) => SECONDARY_HOVER_TEXT_700,
      | (Self::Secondary, 800,) => SECONDARY_HOVER_TEXT_800,
      | (Self::Neutral, 600,) => NEUTRAL_HOVER_TEXT_600,
      | (Self::Neutral, 700,) => NEUTRAL_HOVER_TEXT_700,
      | (Self::Neutral, 800,) => NEUTRAL_HOVER_TEXT_800,
      | _ => panic!("hover_text() step must be 600, 700, or 800"),
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
    assert_eq!(PRIMARY_HOVER_BG, PRIMARY_HOVER_BG_100);
    assert_eq!(PRIMARY_HOVER_TEXT, PRIMARY_HOVER_TEXT_600);
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
    assert_eq!(Palette::Primary.hover_bg(100), PRIMARY_HOVER_BG_100);
    assert_eq!(Palette::Primary.hover_text(600), PRIMARY_HOVER_TEXT_600);
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
      PRIMARY_TEXT_600,
      PRIMARY_BORDER_300,
      PRIMARY_HOVER_BG_100,
      PRIMARY_HOVER_TEXT_600,
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
