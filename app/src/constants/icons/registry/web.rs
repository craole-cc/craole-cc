use crate::prelude::icons::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Actix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod actix {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/actix.svg",)
      .with_link("https://actix.rs",)
      .with_tooltip("Fast, pragmatic, and extensible web framework",)
      .with_label("Actix",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { local() }
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Axum                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod axum {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/tokio.svg",)
      .with_tooltip("",)
      .with_label("Axum",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { local() }
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ HTMX                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod htmx {
  use super::*;

  fn base() -> Icon {
    Icon::new_leptos(icon::SiHtmx,)
      .with_tooltip("",)
      .with_label("HTMX",)
  }

  pub fn default() -> Icon { base() }
  pub fn filled() -> Icon { base() }
  pub fn outlined() -> Icon { base() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Leptos                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod leptos {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/leptos.ico",)
      .with_tooltip("",)
      .with_label("Leptos",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { local() }
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tailwind                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod tailwind {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/tailwind-blue.svg",)
      .with_tooltip("",)
      .with_label("Tailwind CSS",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { local() }
  pub fn outlined() -> Icon { local() }
}
