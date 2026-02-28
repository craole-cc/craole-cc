use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Actix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod actix {
  use super::{
    Icon,
    Variant,
    icon,
  };
  pub struct Actix(pub Variant,);

  impl Actix {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://actix.rs",)
      .with_tooltip("Fast, pragmatic, and extensible web framework",)
      .with_label("Actix",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/actix.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiActix,).colored("brand-actix",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Axum                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod axum {
  use super::{
    Icon,
    Variant,
  };
  pub struct Axum(pub Variant,);

  impl Axum {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://github.com/tokio-rs/axum",)
      .with_tooltip("Ergonomic and modular web framework built with Tokio",)
      .with_label("Axum",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/tokio.svg",) }

  #[must_use]
  pub const fn filled() -> Icon { local() }

  #[must_use]
  pub const fn outlined() -> Icon { local() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ HTMX                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod htmx {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Htmx(pub Variant,);

  impl Htmx {
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://htmx.org",)
      .with_tooltip("High power tools for HTML",)
      .with_label("HTMX",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/htmx.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiHtmx,).colored("brand-htmx",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Leptos                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod leptos {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Leptos(pub Variant,);

  impl Leptos {
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://leptos.dev/",)
      .with_tooltip("A cutting-edge Rust framework for the modern web",)
      .with_label("Leptos",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/leptos.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiLeptos,).colored("brand-leptos",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tailwind CSS                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod tailwind {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Tailwind(pub Variant,);

  impl Tailwind {
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://tailwindcss.com",)
      .with_tooltip("Utility-first CSS framework",)
      .with_label("Tailwind CSS",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/tailwind-blue.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiTailwindcss,)
      .colored("brand-tailwind",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandTailwindOutline,)
      .colored("brand-tailwind",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}
