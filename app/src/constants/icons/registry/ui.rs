use crate::prelude::icons::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Light                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_light {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::BsSun,)
      .with_label("Light theme",)
      .with_tooltip("Switch to light theme",)
  }

  pub fn default() -> Icon { base() }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::BsSun,),) }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::BsSunFill,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme Dark                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_dark {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::BsMoon,)
      .with_label("Dark theme",)
      .with_tooltip("Switch to dark theme",)
  }

  pub fn default() -> Icon { base() }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::BsMoon,),) }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::BsMoonFill,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Theme System                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod theme_system {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::TbSunMoonOutline,)
      .with_label("System theme",)
      .with_tooltip("Use system theme",)
  }

  pub fn default() -> Icon { base() }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::LuSunMoon,),) }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::MdiMonitorDashboard,),) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Closed                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_closed {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::MdiMenuOpen,)
      .with_label("Open menu",)
      .with_tooltip("Open navigation menu",)
  }

  pub fn default() -> Icon { base() }
  pub fn alt() -> Icon { base().with_source(Source::Leptos(icon::CgMenuRight,),) }
  pub fn filled() -> Icon { default() }
  pub fn outlined() -> Icon { default() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Menu Open                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod menu_open {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::MdiMenuClose,)
      .with_label("Close menu",)
      .with_tooltip("Close navigation menu",)
  }

  pub fn default() -> Icon { base() }
  pub fn alt() -> Icon { base().with_source(Source::Leptos(icon::CgMenuLeft,),) }
  pub fn filled() -> Icon { default() }
  pub fn outlined() -> Icon { default() }
}
