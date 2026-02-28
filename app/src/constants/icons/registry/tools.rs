use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Helix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod helix {
  use super::*;

  /// Icon selector for [Helix](https://helix-editor.com/), a post‑modern
  /// modal text editor.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiHelix` with `--brand-helix` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = helix::Helix(Variant::Filled,).get(); 
  /// ```
  pub struct Helix(pub Variant,);

  impl Helix {
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
      .with_link("https://helix-editor.com/",)
      .with_tooltip("Post-modern modal text editor",)
      .with_label("Helix",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/helix.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiHelix`](icon::SiHelix) with `--brand-helix` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiHelix,),)
      .colored("brand-helix",)
  }

  /// Falls back to [`filled`] — no distinct outlined Leptos icon exists.
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Oh My Posh                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ohmyposh {
  use super::*;

  /// Icon selector for [Oh My Posh](https://ohmyposh.dev/), a highly
  /// customisable cross‑platform prompt theme engine.
  ///
  /// Local-only — no upstream Leptos icon exists. All variants resolve
  /// to the bundled SVG.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`local`]   |
  /// | Outlined | [`local`]   |
  pub struct OhMyPosh(pub Variant,);

  impl OhMyPosh {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => local(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://ohmyposh.dev/",)
      .with_tooltip("Prompt theme engine for any shell",)
      .with_label("Oh My Posh",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/ohmyposh.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn filled() -> Icon { local() }

  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Starship                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod starship {
  use super::*;

  /// Icon selector for [Starship](https://starship.rs/), a minimal,
  /// fast, cross‑shell prompt.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiStarship` with `--brand-starship` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  pub struct Starship(pub Variant,);

  impl Starship {
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
      .with_link("https://starship.rs/",)
      .with_tooltip("Minimal, fast, cross-shell prompt",)
      .with_label("Starship",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/starship.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiStarship`](icon::SiStarship) with `--brand-starship` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiStarship,),)
      .colored("brand-starship",)
  }

  /// Falls back to [`filled`] — no distinct outlined Leptos icon exists.
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Typst                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typst {
  use super::*;

  /// Icon selector for [Typst](https://typst.app/), a modern typesetting
  /// system built as a fast alternative to LaTeX.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiTypst` with `--brand-typst` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = typst::Typst(Variant::Filled,).get(); 
  /// ```
  pub struct Typst(pub Variant,);

  impl Typst {
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
      .with_link("https://typst.app/",)
      .with_tooltip("Modern typesetting system",)
      .with_label("Typst",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/typst.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiTypst`](icon::SiTypst) with `--brand-typst` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiTypst,),)
      .colored("brand-typst",)
  }

  /// Falls back to [`filled`] — no distinct outlined Leptos icon exists.
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode {
  use super::*;

  /// Icon selector for [Visual Studio Code](https://code.visualstudio.com/).
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `VsVscode` with `--brand-vscode` |
  /// | Outlined | [`outlined`] — `TbBrandVscodeOutline` with `--brand-vscode` |
  pub struct VsCode(pub Variant,);

  impl VsCode {
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
    Icon::new()
      .with_link("https://code.visualstudio.com/",)
      .with_tooltip("Visual Studio Code editor",)
      .with_label("VS Code",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/vscode.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`VsVscode`](icon::VsVscode) with `--brand-vscode` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::VsVscode,),)
      .colored("brand-vscode",)
  }

  /// Outlined [`TbBrandVscodeOutline`](icon::TbBrandVscodeOutline) with `--brand-vscode` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandVscodeOutline,),)
      .colored("brand-vscode",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code Insiders                                          ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode_insiders {
  use super::*;

  /// Icon selector for [VS Code Insiders](https://code.visualstudio.com/insiders/),
  /// the nightly preview build of Visual Studio Code.
  ///
  /// Mirrors [`vscode`] but uses the Insiders glyph and
  /// `--brand-vscodeinsiders` colour token.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `VsVscodeInsiders` with `--brand-vscodeinsiders` |
  /// | Outlined | [`outlined`] — `TbBrandVscodeOutline` with `--brand-vscodeinsiders` |
  pub struct VsCodeInsiders(pub Variant,);

  impl VsCodeInsiders {
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
    Icon::new_local("icons/logos/vscodium.svg",)
      .with_link("https://code.visualstudio.com/insiders/",)
      .with_tooltip("Visual Studio Code Insiders build",)
      .with_label("VS Code Insiders",)
  }

  /// Local SVG asset with `color-invert` applied for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`VsVscodeInsiders`](icon::VsVscodeInsiders) with `--brand-vscodeinsiders` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::VsVscodeInsiders,),)
      .colored("brand-vscodeinsiders",)
  }

  /// Outlined [`TbBrandVscodeOutline`](icon::TbBrandVscodeOutline) with `--brand-vscodeinsiders`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandVscodeOutline,),)
      .colored("brand-vscodeinsiders",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VSCodium                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscodium {
  use super::*;

  /// Icon selector for [VSCodium](https://vscodium.com/), a community‑driven,
  /// telemetry‑free build of VS Code.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiVscodium` with `--brand-vscodium` |
  /// | Outlined | [`outlined`] — `VsCodeOss` with `--brand-vscodium` |
  pub struct VsCodium(pub Variant,);

  impl VsCodium {
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
    Icon::new_local("icons/logos/vscodium.svg",)
      .with_link("https://vscodium.com/",)
      .with_tooltip("Community-driven, freely-licensed VS Code build",)
      .with_label("VSCodium",)
  }

  /// Local SVG asset with `color-invert` applied for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiVscodium`](icon::SiVscodium) with `--brand-vscodium` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiVscodium,),)
      .colored("brand-vscodium",)
  }

  /// Outlined [`VsCodeOss`](icon::VsCodeOss) with `--brand-vscodium` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::VsCodeOss,),)
      .colored("brand-vscodium",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Zed                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod zed {
  use super::*;

  /// Icon selector for [Zed](https://zed.dev/), a high‑performance,
  /// multiplayer code editor.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiZedindustries` with `--brand-zed` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  pub struct Zed(pub Variant,);

  impl Zed {
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
      .with_link("https://zed.dev/",)
      .with_tooltip("High-performance multiplayer code editor",)
      .with_label("Zed",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/zed.svg",) }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiZedindustries`](icon::SiZedindustries) with `--brand-zed` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiZedindustries,),)
      .colored("brand-zed",)
  }

  /// Falls back to [`filled`] — no distinct outlined Leptos icon exists.
  pub fn outlined() -> Icon { filled() }
}
