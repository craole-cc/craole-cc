use crate::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Actix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod actix {
  use super::*;

  /// Local-only — no Leptos icon available. All variants use the local asset.
  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => local(),
        | Variant::Outlined => local(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/actix.svg",)
      .with_link("https://actix.rs",)
      .with_tooltip("Fast, pragmatic, and extensible web framework",)
      .with_label("Actix",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Axum                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod axum {
  use super::*;

  /// Local-only — uses the Tokio logo as a stand-in. All variants use the
  /// local asset until a dedicated Axum icon is available.
  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => local(),
        | Variant::Outlined => local(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/tokio.svg",)
      .with_link("https://github.com/tokio-rs/axum",)
      .with_tooltip("Ergonomic and modular web framework built with Tokio",)
      .with_label("Axum",)
  }

  /// Local SVG asset (Tokio logo stand-in) — swap when a dedicated asset lands.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ HTMX                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod htmx {
  use super::*;

  /// Leptos-only — no local asset. All variants resolve to the same
  /// Simple Icons icon.
  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => default(),
        | Variant::Local => default(), // no local asset
        | Variant::Filled => default(),
        | Variant::Outlined => default(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::SiHtmx,)
      .with_link("https://htmx.org",)
      .with_tooltip("High power tools for HTML",)
      .with_label("HTMX",)
  }

  /// Canonical default — Simple Icons Leptos icon.
  pub fn default() -> Icon { base() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Leptos                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod leptos {
  use super::*;

  /// No distinct outlined style — `Outlined` falls back to `Filled`.
  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://leptos.dev/",)
      .with_tooltip("A cutting-edge Rust framework for the modern web",)
      .with_label("Leptos",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/leptos.svg",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled Simple Icons icon with `--brand-leptos` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiLeptos,).colored("brand-leptos",) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tailwind CSS                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod tailwind {
  use super::*;

  /// Local-only — no Leptos icon available. All variants use the local asset.
  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => local(),
        | Variant::Outlined => local(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/tailwind-blue.svg",)
      .with_link("https://tailwindcss.com",)
      .with_tooltip("Utility-first CSS framework",)
      .with_label("Tailwind CSS",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}
