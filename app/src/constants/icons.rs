//! # Icon & Logo Types
//!
//! Reusable configuration structs for icon and logo rendering.
//! Now supports both library icons (via `leptos-icons`) and custom SVG assets.

use crate::prelude::*;
use icondata as i;
use leptos_icons::Icon;

//╔═══════════════════════════════════════════════════════════╗
//║ IconSource                                                ║
//╚═══════════════════════════════════════════════════════════╝
/// Determines whether an icon comes from a library or a custom asset.
#[derive(Clone)]
pub enum IconSource {
  /// A named icon from `leptos-icons` (e.g. "Github", "Rust", "Python").
  /// The name is matched to an `icondata` icon at render time.
  Library(&'static str),
  /// A custom SVG asset (fallback using `<img>`).
  Custom(CustomIcon),
}

impl Default for IconSource {
  fn default() -> Self {
    Self::Custom(CustomIcon::default())
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ CustomIcon                                                ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a custom SVG image (used as fallback).
#[derive(Default, Clone)]
pub struct CustomIcon {
  /// Path to the SVG asset (relative to the public root).
  pub source: &'static str,
  /// Optional Tailwind classes (e.g. `"w-6 h-6"`).
  pub class: Option<&'static str>,
  /// Optional inline CSS (e.g. `"filter: brightness(0) invert(0.35);"`).
  pub style: Option<&'static str>,
}

impl CustomIcon {
  pub fn new(source: &'static str) -> Self {
    Self {
      source,
      ..Default::default()
    }
  }

  pub fn with_class(mut self, class: &'static str) -> Self {
    self.class = Some(class);
    self
  }

  pub fn with_style(mut self, style: &'static str) -> Self {
    self.style = Some(style);
    self
  }

  pub fn class(&self) -> &'static str {
    self.class.unwrap_or_default()
  }

  pub fn style(&self) -> &'static str {
    self.style.unwrap_or_default()
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ IconConfig                                                ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a single icon (either library or custom).
#[derive(Default, Clone)]
pub struct IconConfig {
  pub source: IconSource,
  /// Optional Tailwind classes (applies to both library and custom).
  pub class: Option<&'static str>,
  /// Optional inline CSS (applies to both library and custom).
  pub style: Option<&'static str>,
}

impl IconConfig {
  /// Creates a library icon by name.
  pub fn new_library(name: &'static str) -> Self {
    Self {
      source: IconSource::Library(name),
      ..Default::default()
    }
  }

  /// Creates a custom icon from an asset path.
  pub fn new_custom(src: &'static str) -> Self {
    Self {
      source: IconSource::Custom(CustomIcon::new(src)),
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

  /// Helper for `socials.rs` – extracts the source string of a custom icon.
  /// Returns an empty string if the icon is not custom.
  pub fn custom_src(&self) -> &'static str {
    match &self.source {
      IconSource::Custom(c) => c.source,
      _ => "",
    }
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Logo                                                      ║
//╚═══════════════════════════════════════════════════════════╝
/// Configuration for a logo with separate idle and hover states.
#[derive(Default, Clone)]
pub struct Logo {
  /// Idle / default icon.
  pub reset: IconConfig,
  /// Hover icon (defaults to a clone of `reset` if not overridden).
  pub hover: IconConfig,
}

impl Logo {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_icon(reset: IconConfig) -> Self {
    let hover = reset.clone();
    Self { reset, hover }
  }

  pub fn new_icon_src(source: &'static str) -> Self {
    Self::new_icon(IconConfig::new_custom(source))
  }

  pub fn with_hover(mut self, icon: IconConfig) -> Self {
    self.hover = icon;
    self
  }

  pub fn with_icon_src(mut self, source: &'static str) -> Self {
    self.reset = IconConfig::new_custom(source);
    self.hover = IconConfig::new_custom(source);
    self
  }

  pub fn with_icon_class(mut self, class: &'static str) -> Self {
    self.reset.class = Some(class);
    self
  }

  pub fn with_icon_style(mut self, style: &'static str) -> Self {
    self.reset.style = Some(style);
    self
  }

  pub fn with_icon_hover_src(mut self, source: &'static str) -> Self {
    self.hover = IconConfig::new_custom(source);
    self
  }

  pub fn with_icon_hover_class(mut self, class: &'static str) -> Self {
    self.hover.class = Some(class);
    self
  }

  pub fn with_icon_hover_style(mut self, style: &'static str) -> Self {
    self.hover.style = Some(style);
    self
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ RenderIcon – Component to actually draw the icon         ║
//╚═══════════════════════════════════════════════════════════╝
/// Maps a library name to the corresponding `icondata` icon.
fn name_to_icon(name: &str) -> Option<i::Icon> {
  match name {
    "Github" => Some(i::FaGithubBrands),
    "Htmx" => Some(i::SiHtmx),
    "Actix" => Some(i::SiActix),
    "Rust" => Some(i::FaRustBrands),
    "Python" => Some(i::FaPythonBrands),
    "Git" => Some(i::FaGitAltBrands),
    // Add more mappings as needed
    _ => None,
  }
}

/// Renders either a library icon (from leptos-icons) or a custom <img>.
#[component]
pub fn RenderIcon(icon: IconConfig) -> impl IntoView {
  match icon.source {
    IconSource::Library(name) => {
      if let Some(icondata_icon) = name_to_icon(name) {
        // Library icons use currentColor – dark mode handled by parent dark:text-white
        view! { <Icon icon=icondata_icon style=icon.style() /> }.into_any()
      } else {
        view! { <span></span> }.into_any()
      }
    }
    IconSource::Custom(custom) => {
      // Apply the icon's class (which may include dark:invert) directly to the img
      view! {
        <img
          src=custom.source
          class=custom.class()
          style=custom.style()
          alt=""
          loading="lazy"
          onerror="this.style.display='none'"
        />
      }
      .into_any()
    }
  }
}
