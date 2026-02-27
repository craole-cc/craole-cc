use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Light                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_light {
  use super::*;

  /// Icon selector for the light theme control.
  ///
  /// `Default` and `Local` resolve to [`outlined`] — the stroke sun is the
  /// neutral resting state before the user has expressed a preference.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`outlined`] — `BsSun` stroke |
  /// | `Local` | [`outlined`] — no local asset for UI icons |
  /// | `Filled` | [`filled`] — `BsSunFill` solid |
  /// | `Outlined` | [`outlined`] — `BsSun` stroke |
  ///
  /// # Example
  /// ```rust
  /// let icon = theme_light::ThemeLight(Variant::Filled,).get(); 
  /// ```
  pub struct ThemeLight(pub Variant,);

  impl ThemeLight {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => outlined(),
        | Variant::Local => outlined(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::RiSunWeatherFill,)
      .with_label("Light theme",)
      .with_tooltip("Switch to light theme",)
  }

  pub fn default() -> Icon { outlined() }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiSunWeatherLine,),) }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiSunWeatherFill,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Dark                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_dark {
  use super::*;

  /// Icon selector for the dark theme control.
  ///
  /// `Default` and `Local` resolve to [`outlined`] — the stroke moon is the
  /// neutral resting state before the user has expressed a preference.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`outlined`] — `BsMoon` stroke |
  /// | `Local` | [`outlined`] — no local asset for UI icons |
  /// | `Filled` | [`filled`] — `BsMoonFill` solid |
  /// | `Outlined` | [`outlined`] — `BsMoon` stroke |
  ///
  /// # Example
  /// ```rust
  /// let icon = theme_dark::ThemeDark(Variant::Filled,).get(); 
  /// ```
  pub struct ThemeDark(pub Variant,);

  impl ThemeDark {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => outlined(),
        | Variant::Local => outlined(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::BsMoon,)
      .with_label("Dark theme",)
      .with_tooltip("Switch to dark theme",)
  }

  /// Canonical default — resolves to [`outlined`].
  pub fn default() -> Icon { outlined() }

  /// Bootstrap outlined [`BsMoon`](icon::BsMoon) stroke icon.
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiMoonClearWeatherLine,),) }

  /// Bootstrap filled [`BsMoonFill`](icon::BsMoonFill) solid icon.
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiMoonClearWeatherFill,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme System                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_system {
  use super::*;

  /// Icon selector for the system theme control.
  ///
  /// `Filled` uses a monitor dashboard icon to convey OS-level control.
  /// `Outlined` uses the Lucide sun-moon for a lighter feel.
  /// `Default` resolves to the Tabler sun-moon outline.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`default`] — `TbSunMoonOutline` |
  /// | `Local` | [`default`] — no local asset for UI icons |
  /// | `Filled` | [`filled`] — `MdiMonitorDashboard` |
  /// | `Outlined` | [`outlined`] — `LuSunMoon` |
  ///
  /// # Example
  /// ```rust
  /// let icon = theme_system::ThemeSystem(Variant::Filled,).get(); 
  /// ```
  pub struct ThemeSystem(pub Variant,);

  impl ThemeSystem {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => default(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::TbSunMoonOutline,)
      .with_label("System theme",)
      .with_tooltip("Use system theme",)
  }

  /// Lucide [`LuSunMoon`](icon::LuSunMoon) outlined icon — lighter feel.
  pub fn default() -> Icon { base().with_source(Source::Leptos(icon::MdiMonitorDashboard,),) }

  /// Canonical default — Tabler [`TbSunMoonOutline`](icon::TbSunMoonOutline).
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::TbSunMoonOutline,),) }

  /// Material Design [`MdiMonitorDashboard`](icon::MdiMonitorDashboard) — conveys OS-level control.
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::MdiMonitorDashboard,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Closed                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_closed {
  use super::*;

  /// Icon selector for the menu-closed (hamburger) control.
  ///
  /// `Filled` and `Outlined` both resolve to [`default`] — this icon has no
  /// meaningful fill/stroke distinction. For the right-aligned hamburger
  /// style use [`MenuClosedExt`] with [`Extended::Alt`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`default`] — `MdiMenuOpen` |
  /// | `Local` | [`default`] — no local asset for UI icons |
  /// | `Filled` | [`default`] — no distinct filled style |
  /// | `Outlined` | [`default`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = menu_closed::MenuClosed(Variant::Default,).get(); 
  /// ```
  pub struct MenuClosed(pub Variant,);

  impl MenuClosed {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => default(),
        | Variant::Filled => default(),
        | Variant::Outlined => default(),
      }
    }
  }

  /// Extended variant selector for non-standard menu-closed styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Alt` | [`alt`] — `CgMenuRight` right-aligned hamburger |
  ///
  /// # Example
  /// ```rust
  /// let icon = menu_closed::MenuClosedExt(menu_closed::Extended::Alt,).get(); 
  /// ```
  pub struct MenuClosedExt(pub Extended,);

  impl MenuClosedExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::Alt => alt(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Right-aligned hamburger — use when the menu opens from the right.
    Alt,
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::MdiMenuOpen,)
      .with_label("Open menu",)
      .with_tooltip("Open navigation menu",)
  }

  /// Canonical default — Material Design [`MdiMenuOpen`](icon::MdiMenuOpen).
  pub fn default() -> Icon { base() }
  /// Right-aligned [`CgMenuRight`](icon::CgMenuRight) hamburger icon.
  pub fn alt() -> Icon { base().with_source(Source::Leptos(icon::CgMenuRight,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Open                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_open {
  use super::*;

  /// Icon selector for the menu-open (close/dismiss) control.
  ///
  /// `Filled` and `Outlined` both resolve to [`default`] — this icon has no
  /// meaningful fill/stroke distinction. For the left-aligned close style
  /// use [`MenuOpenExt`] with [`Extended::Alt`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`default`] — `MdiMenuClose` |
  /// | `Local` | [`default`] — no local asset for UI icons |
  /// | `Filled` | [`default`] — no distinct filled style |
  /// | `Outlined` | [`default`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = menu_open::MenuOpen(Variant::Default,).get(); 
  /// ```
  pub struct MenuOpen(pub Variant,);

  impl MenuOpen {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => default(),
        | Variant::Filled => default(),
        | Variant::Outlined => default(),
      }
    }
  }

  /// Extended variant selector for non-standard menu-open styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Alt` | [`alt`] — `CgMenuLeft` left-aligned close icon |
  ///
  /// # Example
  /// ```rust
  /// let icon = menu_open::MenuOpenExt(menu_open::Extended::Alt,).get(); 
  /// ```
  pub struct MenuOpenExt(pub Extended,);

  impl MenuOpenExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::Alt => alt(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Left-aligned close icon — use when the menu opens from the left.
    Alt,
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::MdiMenuClose,)
      .with_label("Close menu",)
      .with_tooltip("Close navigation menu",)
  }

  /// Canonical default — Material Design [`MdiMenuClose`](icon::MdiMenuClose).
  pub fn default() -> Icon { base() }
  /// Left-aligned [`CgMenuLeft`](icon::CgMenuLeft) close icon.
  pub fn alt() -> Icon { base().with_source(Source::Leptos(icon::CgMenuLeft,),) }
}
