use crate::prelude::{
  icons::{
    theme_dark,
    theme_light,
    theme_system,
  },
  *,
};

/// The three theme modes the application supports.
///
/// `System` defers to the OS `prefers-color-scheme` media query at runtime.
/// Cycling order: `System` → `Light` → `Dark` → `System`.
#[derive(Clone, Copy, PartialEq, Eq, Default,)]
pub enum Theme {
  /// Follow the OS colour-scheme preference.
  #[default]
  System,
  /// Force light mode regardless of OS preference.
  Light,
  /// Force dark mode regardless of OS preference.
  Dark,
}

impl Theme {
  /// Returns the next theme in the cycle: System → Light → Dark → System.
  #[must_use]
  pub const fn next(self,) -> Self {
    match self {
      | Self::System => Self::Light,
      | Self::Light => Self::Dark,
      | Self::Dark => Self::System,
    }
  }

  /// Human-readable label for the *current* theme state.
  #[must_use]
  pub const fn label(self,) -> &'static str {
    match self {
      | Self::System => "System theme",
      | Self::Light => "Light theme",
      | Self::Dark => "Dark theme",
    }
  }

  /// Tooltip hint describing what the *next* action will do.
  #[must_use]
  pub const fn next_label(self,) -> &'static str {
    match self {
      | Self::System => "Switch to light theme",
      | Self::Light => "Switch to dark theme",
      | Self::Dark => "Switch to system theme",
    }
  }

  /// Resolves `System` to the actual `"light"` or `"dark"` string
  /// by reading `window.matchMedia` in the browser.
  ///
  /// Falls back to `"dark"` on the server (SSR / non-hydrate builds).
  #[must_use]
  pub fn resolve(self,) -> &'static str {
    match self {
      | Self::Light => "light",
      | Self::Dark => "dark",
      | Self::System => {
        #[cfg(feature = "hydrate")]
        {
          let prefers_dark = window()
            .and_then(|w| {
              w.match_media("(prefers-color-scheme: dark)",)
                .ok()
                .flatten()
            },)
            .is_some_and(|mql : MediaQueryList| mql.matches(),);
          if prefers_dark { "dark" } else { "light" }
        }
        #[cfg(not(feature = "hydrate"))]
        "dark"
      }
    }
  }

  /// Returns the outlined icon for the toggle button.
  ///
  /// Always outlined regardless of active state — the button is a control,
  /// not an indicator. Use [`icons`] for the dropdown where filled = active.
  #[must_use]
  pub fn icon(self,) -> Icon {
    match self {
      | Self::System => theme_system::outlined(),
      | Self::Light => theme_light::outlined(),
      | Self::Dark => theme_dark::outlined(),
    }
  }

  /// Returns the icon pair (rest, active) for this theme variant.
  ///
  /// - `rest` — outlined style, used in the toggle button when this theme is *not* currently
  ///   active.
  /// - `active` — filled style, used when this theme *is* active.
  #[must_use]
  pub fn icons(self,) -> (Icon, Icon,) {
    match self {
      | Self::System => (theme_system::default(), theme_system::filled(),),
      | Self::Light => (theme_light::outlined(), theme_light::filled(),),
      | Self::Dark => (theme_dark::outlined(), theme_dark::filled(),),
    }
  }
}
