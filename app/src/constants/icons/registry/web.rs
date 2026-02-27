use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Actix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod actix {
  use super::*;

  /// Icon selector for [Actix](https://actix.rs), a fast, pragmatic,
  /// and extensible Rust web framework.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiActix` with `--brand-actix` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  pub struct Actix(pub Variant,);

  impl Actix {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
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
    Icon::new_local("icons/logos/actix.svg",)
      .with_link("https://actix.rs",)
      .with_tooltip("Fast, pragmatic, and extensible web framework",)
      .with_label("Actix",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiActix`](icon::SiActix) with `--brand-actix` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiActix,),)
      .colored("brand-actix",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ HTMX                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod htmx {
  use super::*;

  /// Icon selector for [HTMX](https://htmx.org), a library that gives
  /// HTML access to modern browser features without JavaScript.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`] — no local asset, resolves via Leptos |
  /// | Local    | [`filled`] — no local asset |
  /// | Filled   | [`filled`] — `SiHtmx` with `--brand-htmx` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  pub struct Htmx(pub Variant,);

  impl Htmx {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::SiHtmx,)
      .with_link("https://htmx.org",)
      .with_tooltip("High power tools for HTML",)
      .with_label("HTMX",)
  }

  /// Canonical default — resolves to [`filled`]; no local asset available.
  pub fn default() -> Icon { filled() }

  /// Filled [`SiHtmx`](icon::SiHtmx) with `--brand-htmx` colour.
  pub fn filled() -> Icon { base().colored("brand-htmx",) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Leptos                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod leptos {
  use super::*;

  /// Icon selector for [Leptos](https://leptos.dev/), a cutting-edge
  /// full-stack Rust web framework.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiLeptos` with `--brand-leptos` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  pub struct Leptos(pub Variant,);

  impl Leptos {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
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

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiLeptos`](icon::SiLeptos) with `--brand-leptos` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiLeptos,).colored("brand-leptos",) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Tailwind CSS                                              ║
//╚═══════════════════════════════════════════════════════════╝
pub mod tailwind {
  use super::*;

  /// Icon selector for [Tailwind CSS](https://tailwindcss.com), a
  /// utility-first CSS framework for rapid UI development.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiTailwindcss` with `--brand-tailwind` |
  /// | Outlined | [`outlined`] — `TbBrandTailwindOutline` with `--brand-tailwind` |
  pub struct Tailwind(pub Variant,);

  impl Tailwind {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
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
    Icon::new_local("icons/logos/tailwind-blue.svg",)
      .with_link("https://tailwindcss.com",)
      .with_tooltip("Utility-first CSS framework",)
      .with_label("Tailwind CSS",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiTailwindcss`](icon::SiTailwindcss) with `--brand-tailwind` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiTailwindcss,),)
      .colored("brand-tailwind",)
  }

  /// Outlined [`TbBrandTailwindOutline`](icon::TbBrandTailwindOutline) with `--brand-tailwind`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandTailwindOutline,),)
      .colored("brand-tailwind",)
  }
}
