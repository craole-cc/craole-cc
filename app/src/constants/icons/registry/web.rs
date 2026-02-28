// web.rs
use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Actix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod actix {
  use super::*;

  /// Variant selector for the Actix icon.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// ```rust
  /// let icon = actix::Actix(Variant::Filled,).get(); 
  /// ```
  pub struct Actix(pub Variant,);

  impl Actix {
    /// Resolves this variant to a concrete [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => filled(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://actix.rs",)
      .with_tooltip("Fast, pragmatic, and extensible web framework",)
      .with_label("Actix",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/actix.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiActix`] with `--brand-actix` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiActix,).colored("brand-actix",) }

  /// Outlined — no distinct style exists, falls back to [`filled`].
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Axum                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod axum {
  use super::*;

  /// Variant selector for the Axum icon.
  ///
  /// No distinct outlined style and no dedicated Leptos icon — all
  /// variants resolve to the local Tokio SVG asset.
  ///
  /// ```rust
  /// let icon = axum::Axum(Variant::Local,).get(); 
  /// ```
  pub struct Axum(pub Variant,);

  impl Axum {
    /// Resolves this variant to a concrete [`Icon`].
    /// All variants fall back to [`local`] — no Leptos icon or outlined
    /// style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => local(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://github.com/tokio-rs/axum",)
      .with_tooltip("Ergonomic and modular web framework built with Tokio",)
      .with_label("Axum",)
  }

  /// Local SVG asset (Tokio mark) — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/tokio.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled — no distinct style exists, falls back to [`local`].
  pub fn filled() -> Icon { local() }

  /// Outlined — no distinct style exists, falls back to [`local`].
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ HTMX                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod htmx {
  use super::*;

  /// Variant selector for the HTMX icon.
  ///
  /// No local asset and no distinct outlined style — all variants resolve
  /// to [`filled`].
  ///
  /// ```rust
  /// let icon = htmx::Htmx(Variant::Filled,).get(); 
  /// ```
  pub struct Htmx(pub Variant,);

  impl Htmx {
    /// Resolves this variant to a concrete [`Icon`].
    /// All variants fall back to [`filled`] — no local asset or outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://htmx.org",)
      .with_tooltip("High power tools for HTML",)
      .with_label("HTMX",)
  }

  /// Local asset unavailable — falls back to [`filled`].
  pub fn local() -> Icon { base().via_local("icons/logos/htmx.svg",) }

  /// Canonical default — no local asset, resolves to [`filled`].
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiHtmx`] with `--brand-htmx` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiHtmx,).colored("brand-htmx",) }

  /// Outlined — no distinct style exists, falls back to [`filled`].
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Leptos                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod leptos {
  use super::*;

  /// Variant selector for the Leptos icon.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// ```rust
  /// let icon = leptos::Leptos(Variant::Filled,).get(); 
  /// ```
  pub struct Leptos(pub Variant,);

  impl Leptos {
    /// Resolves this variant to a concrete [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
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

  /// Filled [`icon::SiLeptos`] with `--brand-leptos` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiLeptos,).colored("brand-leptos",) }

  /// Outlined — no distinct style exists, falls back to [`filled`].
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tailwind CSS                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod tailwind {
  use super::*;

  /// Variant selector for the Tailwind CSS icon.
  ///
  /// ```rust
  /// let icon = tailwind::Tailwind(Variant::Outlined,).get(); 
  /// ```
  pub struct Tailwind(pub Variant,);

  impl Tailwind {
    /// Resolves this variant to a concrete [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://tailwindcss.com",)
      .with_tooltip("Utility-first CSS framework",)
      .with_label("Tailwind CSS",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/tailwind-blue.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiTailwindcss`] with `--brand-tailwind` colour.
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiTailwindcss,)
      .colored("brand-tailwind",)
  }

  /// Outlined [`icon::TbBrandTailwindOutline`] with `--brand-tailwind` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandTailwindOutline,)
      .colored("brand-tailwind",)
  }
}
