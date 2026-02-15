//! # Color System
//!
//! Centralized color definitions for consistent theming across the application.
//! All colors are automatically light/dark mode aware.
//!
//! ## Usage
//!
//! ```rust
//! use crate::components::colors::*;
//!
//! // Use in components - automatically adapts to theme
//! <Divider config=Divider::default().with_dot(PRIMARY_ACCENT) />
//! <span class=PRIMARY_TEXT>"Highlighted text"</span>
//! ```

/// Primary brand color base (e.g., "teal-400")
pub const PRIMARY: &str = "teal-400";

/// Secondary brand color base (e.g., "purple-400")
pub const SECONDARY: &str = "purple-400";

/// Tertiary brand color base (e.g., "slate-400")
pub const TERTIARY: &str = "slate-400";

// Accent colors - automatically adapts between light/dark modes
/// Primary accent with opacity (badges, dots, pills, highlights)
pub const PRIMARY_ACCENT: &str = "bg-teal-400/70 dark:bg-teal-500/70";

/// Secondary accent with opacity
pub const SECONDARY_ACCENT: &str = "bg-purple-400/70 dark:bg-purple-500/70";

/// Tertiary accent with opacity
pub const TERTIARY_ACCENT: &str = "bg-slate-400/70 dark:bg-slate-500/70";

/// Primary accent (lighter variant)
pub const PRIMARY_ACCENT_LIGHT: &str = "bg-teal-400/50 dark:bg-teal-500/50";

/// Secondary accent (lighter variant)
pub const SECONDARY_ACCENT_LIGHT: &str = "bg-purple-400/50 dark:bg-purple-500/50";

/// Tertiary accent (lighter variant)
pub const TERTIARY_ACCENT_LIGHT: &str = "bg-slate-400/50 dark:bg-slate-500/50";

/// Primary accent (stronger variant)
pub const PRIMARY_ACCENT_STRONG: &str = "bg-teal-400/90 dark:bg-teal-500/90";

/// Secondary accent (stronger variant)
pub const SECONDARY_ACCENT_STRONG: &str = "bg-purple-400/90 dark:bg-purple-500/90";

/// Tertiary accent (stronger variant)
pub const TERTIARY_ACCENT_STRONG: &str = "bg-slate-400/90 dark:bg-slate-500/90";

// Text colors - automatically adapts
/// Primary text color
pub const PRIMARY_TEXT: &str = "text-teal-600 dark:text-teal-400";

/// Secondary text color
pub const SECONDARY_TEXT: &str = "text-purple-600 dark:text-purple-400";

/// Tertiary text color
pub const TERTIARY_TEXT: &str = "text-slate-600 dark:text-slate-400";

// Background colors (subtle fills) - automatically adapts
/// Primary background (subtle)
pub const PRIMARY_BG: &str = "bg-teal-100 dark:bg-teal-900/30";

/// Secondary background (subtle)
pub const SECONDARY_BG: &str = "bg-purple-100 dark:bg-purple-900/30";

/// Tertiary background (subtle)
pub const TERTIARY_BG: &str = "bg-slate-100 dark:bg-slate-800/50";

// Solid backgrounds - automatically adapts
/// Primary solid background
pub const PRIMARY_BG_SOLID: &str = "bg-teal-500 dark:bg-teal-600";

/// Secondary solid background
pub const SECONDARY_BG_SOLID: &str = "bg-purple-500 dark:bg-purple-600";

/// Tertiary solid background
pub const TERTIARY_BG_SOLID: &str = "bg-slate-500 dark:bg-slate-600";

// Border colors - automatically adapts
/// Primary border
pub const PRIMARY_BORDER: &str = "border-teal-400 dark:border-teal-600";

/// Secondary border
pub const SECONDARY_BORDER: &str = "border-purple-400 dark:border-purple-600";

/// Tertiary border
pub const TERTIARY_BORDER: &str = "border-slate-300 dark:border-slate-700";

// Ring colors (for focus states) - automatically adapts
/// Primary ring for focus states
pub const PRIMARY_RING: &str = "ring-teal-400 dark:ring-teal-500";

/// Secondary ring for focus states
pub const SECONDARY_RING: &str = "ring-purple-400 dark:ring-purple-500";

/// Tertiary ring for focus states
pub const TERTIARY_RING: &str = "ring-slate-400 dark:ring-slate-500";

// Hover states - automatically adapts
/// Primary hover background
pub const PRIMARY_HOVER: &str = "hover:bg-teal-50 dark:hover:bg-teal-900/20";

/// Secondary hover background
pub const SECONDARY_HOVER: &str = "hover:bg-purple-50 dark:hover:bg-purple-900/20";

/// Tertiary hover background
pub const TERTIARY_HOVER: &str = "hover:bg-slate-50 dark:hover:bg-slate-800/30";

/// Color palette configuration for dynamic theming
#[derive(Clone, Copy, Debug)]
pub struct ColorPalette {
  pub primary: &'static str,
  pub secondary: &'static str,
  pub tertiary: &'static str,
}

impl Default for ColorPalette {
  fn default() -> Self {
    Self {
      primary: PRIMARY,
      secondary: SECONDARY,
      tertiary: TERTIARY,
    }
  }
}

impl ColorPalette {
  /// Creates a custom color palette
  pub fn new(primary: &'static str, secondary: &'static str, tertiary: &'static str) -> Self {
    Self {
      primary,
      secondary,
      tertiary,
    }
  }

  /// Returns primary accent with dark mode support
  pub fn primary_accent(&self) -> String {
    let base = self.primary.replace("-400", "");
    format!("bg-{}-400/70 dark:bg-{}-500/70", base, base)
  }

  /// Returns secondary accent with dark mode support
  pub fn secondary_accent(&self) -> String {
    let base = self.secondary.replace("-400", "");
    format!("bg-{}-400/70 dark:bg-{}-500/70", base, base)
  }

  /// Returns tertiary accent with dark mode support
  pub fn tertiary_accent(&self) -> String {
    let base = self.tertiary.replace("-400", "");
    format!("bg-{}-400/70 dark:bg-{}-500/70", base, base)
  }
}
