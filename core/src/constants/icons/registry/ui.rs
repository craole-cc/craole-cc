use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Light                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_light {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct ThemeLight(pub Variant,);

  impl ThemeLight {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
        | Variant::Default | Variant::Local => default(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_label("Light theme",)
      .with_tooltip("Switch to light theme",)
  }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::RiSunWeatherFill,) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::RiSunWeatherLine,) }

  #[must_use]
  pub fn default() -> Icon { outlined() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Dark                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_dark {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct ThemeDark(pub Variant,);

  impl ThemeDark {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
        | Variant::Default | Variant::Local => default(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_label("Dark theme",)
      .with_tooltip("Switch to dark theme",)
  }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::RiMoonClearWeatherFill,) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::RiMoonClearWeatherLine,) }

  #[must_use]
  pub fn default() -> Icon { outlined() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme System                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_system {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct ThemeSystem(pub Variant,);

  impl ThemeSystem {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
        | Variant::Default | Variant::Local => default(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_label("System theme",)
      .with_tooltip("Use system theme",)
  }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::MdiMonitorDashboard,) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::TbSunMoonOutline,) }

  #[must_use]
  pub fn default() -> Icon { outlined() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Open                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_open {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct MenuOpen(pub Variant,);

  impl MenuOpen {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => default(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_label("Open menu",)
      .with_tooltip("Open navigation menu",)
  }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::MdiMenuOpen,) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::CgMenuRight,) }

  #[must_use]
  pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Close                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_close {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct MenuClose(pub Variant,);

  impl MenuClose {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => default(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_label("Close menu",)
      .with_tooltip("Close navigation menu",)
  }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::MdiMenuClose,) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::CgMenuLeft,) }

  #[must_use]
  pub fn default() -> Icon { filled() }
}
