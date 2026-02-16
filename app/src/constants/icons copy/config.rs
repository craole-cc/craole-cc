//! Core types for icons – supports Leptos icons (icondata) and local SVG assets.

use super::ico;

//╔═══════════════════════════════════════════════════════════╗
//║ Source – where the icon comes from                        ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub enum Source {
  #[default]
  Empty,
  Leptos(ico::Icon),
  Local(&'static str),
}

//╔═══════════════════════════════════════════════════════════╗
//║ Icon – a single icon with optional styling                ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub struct Icon {
  pub source: Source,
  // pub class: &'static str,
  // pub style: &'static str,
  pub link: &'static str,
  pub label: &'static str,
  pub tooltip: &'static str,
  pub variants: Variants,
}

impl Icon {
  pub fn new() -> Self {
    Self::default()
  }

  /// Chainable setter to convert this Icon into a Leptos icon.
  pub fn from_leptos(mut self, icon: ico::Icon) -> Self {
    self.source = Source::Leptos(icon);
    self
  }

  /// Constructor for a Leptos icon (convenience).
  pub fn new_leptos(icon: ico::Icon) -> Self {
    Self::new().from_leptos(icon)
  }

  /// Chainable setter to convert this Icon into a local asset.
  pub fn from_local(mut self, src: &'static str) -> Self {
    self.source = Source::Local(src);
    self
  }

  /// Constructor for a local asset (convenience).
  pub fn new_local(src: &'static str) -> Self {
    Self::new().from_local(src)
  }

  /// Add Tailwind utility classes.
  pub fn with_tailwind(mut self, class: &'static str) -> Self {
    self.class = class;
    self
  }

  /// Add inline CSS styles.
  pub fn with_css(mut self, style: &'static str) -> Self {
    self.style = style;
    self
  }
}

// Automatic conversions for ergonomics.
impl From<&'static str> for Icon {
  fn from(src: &'static str) -> Self {
    Icon::new_local(src)
  }
}

impl From<ico::Icon> for Icon {
  fn from(icon: ico::Icon) -> Self {
    Icon::new_leptos(icon)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Set – light/dark pair                                     ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub struct Variant {
  pub light: Icon,
  pub dark: Icon,
}

impl Variant {
  pub fn new() -> Self {
    Self::default()
  }

  /// Create a Set where both light and dark use the same Leptos icon.
  pub fn from_leptos(icon: ico::Icon) -> Self {
    let icon = Icon::new_leptos(icon);
    Self {
      light: icon.clone(),
      dark: icon,
    }
  }

  /// Create a Set where both light and dark use the same local asset.
  pub fn from_local(src: &'static str) -> Self {
    let icon = Icon::new_local(src);
    Self {
      light: icon.clone(),
      dark: icon,
    }
  }

  /// Create a Set with separate light and dark local assets.
  pub fn from_local_pair(light_src: &'static str, dark_src: &'static str) -> Self {
    Self {
      light: Icon::new_local(light_src),
      dark: Icon::new_local(dark_src),
    }
  }

  /// Set the light icon (accepts anything convertible to `Icon`).
  pub fn with_light(mut self, icon: impl Into<Icon>) -> Self {
    self.light = icon.into();
    self
  }

  /// Set the dark icon (accepts anything convertible to `Icon`).
  pub fn with_dark(mut self, icon: impl Into<Icon>) -> Self {
    self.dark = icon.into();
    self
  }

  /// Apply the same Tailwind classes to both light and dark icons.
  pub fn with_tailwind(mut self, class: &'static str) -> Self {
    self.light = self.light.with_tailwind(class);
    self.dark = self.dark.with_tailwind(class);
    self
  }

  /// Apply Tailwind classes to the light icon only.
  pub fn with_light_tailwind(mut self, class: &'static str) -> Self {
    self.light = self.light.with_tailwind(class);
    self
  }

  /// Apply Tailwind classes to the dark icon only.
  pub fn with_dark_tailwind(mut self, class: &'static str) -> Self {
    self.dark = self.dark.with_tailwind(class);
    self
  }

  /// Apply the same inline CSS to both light and dark icons.
  pub fn with_css(mut self, style: &'static str) -> Self {
    self.light = self.light.with_css(style);
    self.dark = self.dark.with_css(style);
    self
  }

  /// Apply CSS to the light icon only.
  pub fn with_light_css(mut self, style: &'static str) -> Self {
    self.light = self.light.with_css(style);
    self
  }

  /// Apply CSS to the dark icon only.
  pub fn with_dark_css(mut self, style: &'static str) -> Self {
    self.dark = self.dark.with_css(style);
    self
  }
}

#[derive(Default, Clone)]
pub struct State {
  pub default: Icon,
  pub hover: Icon,
  pub active: Icon,
  pub disabled: Icon,
}
