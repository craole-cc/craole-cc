use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Bash                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod bash {
  use super::*;

  /// Variant selector for the Bash icon.
  ///
  /// Wrap a [`Variant`] and call `.get()` to resolve the [`Icon`]:
  /// ```rust
  /// let icon = bash::Bash(Variant::Filled,).get(); 
  /// ```
  pub struct Bash(pub Variant,);

  impl Bash {
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
      .with_link("https://www.gnu.org/software/bash/",)
      .with_tooltip("Bourne Again Shell - Unix shell and command language",)
      .with_label("Bash",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/bash.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiGnubash`] with `--brand-bash` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiGnubash,).colored("brand-bash",) }

  /// Outlined [`icon::MdiBash`] with `--brand-bash` colour.
  pub fn outlined() -> Icon { base().via_leptos(icon::MdiBash,).colored("brand-bash",) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ JavaScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod javascript {
  use super::*;

  /// Variant selector for the JavaScript icon.
  ///
  /// ```rust
  /// let icon = javascript::JavaScript(Variant::Outlined,).get(); 
  /// ```
  pub struct JavaScript(pub Variant,);

  impl JavaScript {
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
      .with_link("https://www.javascript.com/",)
      .with_tooltip("Lightweight, interpreted programming language for the web",)
      .with_label("JavaScript",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/javascript.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiJavascript`] with `--brand-javascript` colour.
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiJavascript,)
      .colored("brand-javascript",)
  }

  /// Outlined [`icon::TbBrandJavascriptOutline`] with `--brand-javascript` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandJavascriptOutline,)
      .colored("brand-javascript",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nushell                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nushell {
  use super::*;

  /// Variant selector for the Nushell icon.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// ```rust
  /// let icon = nushell::Nushell(Variant::Filled,).get(); 
  /// ```
  pub struct Nushell(pub Variant,);

  impl Nushell {
    /// Resolves this variant to a concrete [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(), // no distinct outlined — falls back
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.nushell.sh",)
      .with_tooltip("A modern shell and scripting language",)
      .with_label("Nushell",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/nushell.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiNushell`] with `--brand-nushell` colour.
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiNushell,)
      .colored("brand-nushell",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ POSIX                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod posix {
  use super::*;

  /// Variant selector for the POSIX Shell icon.
  ///
  /// Reuses GNU Bash icons with `--brand-posix` colouring since no
  /// dedicated POSIX icon exists upstream.
  ///
  /// ```rust
  /// let icon = posix::Posix(Variant::Default,).get(); 
  /// ```
  pub struct Posix(pub Variant,);

  impl Posix {
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
      .with_link("https://pubs.opengroup.org/onlinepubs/9799919799/",)
      .with_tooltip("Portable Operating System Interface shell standard",)
      .with_label("POSIX Shell",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/posix-shell.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiGnubash`] with `--brand-posix` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiGnubash,).colored("brand-posix",) }

  /// Outlined [`icon::MdiBash`] with `--brand-posix` colour.
  pub fn outlined() -> Icon { base().via_leptos(icon::MdiBash,).colored("brand-posix",) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PowerShell                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod powershell {
  use super::*;

  /// Variant selector for the PowerShell icon.
  ///
  /// ```rust
  /// let icon = powershell::PowerShell(Variant::Outlined,).get(); 
  /// ```
  pub struct PowerShell(pub Variant,);

  impl PowerShell {
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
      .with_link("https://learn.microsoft.com/en-us/powershell/",)
      .with_tooltip("Task automation and configuration management framework",)
      .with_label("PowerShell",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/powershell.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::MdiPowershell`] with `--brand-powershell` colour.
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::MdiPowershell,)
      .colored("brand-powershell",)
  }

  /// Outlined [`icon::TbBrandPowershellOutline`] with `--brand-powershell` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandPowershellOutline,)
      .colored("brand-powershell",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Python                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod python {
  use super::*;

  /// Variant selector for the Python icon.
  ///
  /// ```rust
  /// let icon = python::Python(Variant::Filled,).get(); 
  /// ```
  pub struct Python(pub Variant,);

  impl Python {
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
      .with_link("https://www.python.org/",)
      .with_tooltip("Versatile, high-level programming language",)
      .with_label("Python",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/python.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiPython`] with `--brand-python` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiPython,).colored("brand-python",) }

  /// Outlined [`icon::AiPythonOutlined`] with `--brand-python` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::AiPythonOutlined,)
      .colored("brand-python",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Rust                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod rust {
  use super::*;

  /// Variant selector for the Rust icon.
  ///
  /// ```rust
  /// let icon = rust::Rust(Variant::Filled,).get(); 
  /// ```
  pub struct Rust(pub Variant,);

  impl Rust {
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
      .with_link("https://www.rust-lang.org/",)
      .with_tooltip("Systems programming language focused on safety and performance",)
      .with_label("Rust",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/rust.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiRust`] with `--brand-rust` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiRust,).colored("brand-rust",) }

  /// Outlined [`icon::TbBrandRustOutline`] with `--brand-rust` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandRustOutline,)
      .colored("brand-rust",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ TypeScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typescript {
  use super::*;

  /// Variant selector for the TypeScript icon.
  ///
  /// ```rust
  /// let icon = typescript::TypeScript(Variant::Outlined,).get(); 
  /// ```
  pub struct TypeScript(pub Variant,);

  impl TypeScript {
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
      .with_link("https://www.typescriptlang.org/",)
      .with_tooltip("Strongly typed superset of JavaScript",)
      .with_label("TypeScript",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base().via_local("icons/logos/typescript.svg",) }

  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }

  /// Filled [`icon::SiTypescript`] with `--brand-typescript` colour.
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiTypescript,)
      .colored("brand-typescript",)
  }

  /// Outlined [`icon::TbBrandTypescriptOutline`] with `--brand-typescript` colour.
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandTypescriptOutline,)
      .colored("brand-typescript",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Zig                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod zig {
  use super::*;

  /// Variant selector for the Zig icon.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// The local asset requires `color-invert` for dark mode compatibility
  /// because the upstream SVG uses a black fill with no theming.
  ///
  /// ```rust
  /// let icon = zig::Zig(Variant::Local,).get(); 
  /// ```
  pub struct Zig(pub Variant,);

  impl Zig {
    /// Resolves this variant to a concrete [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(), // no distinct outlined — falls back
      }
    }
  }

  fn base() -> Icon {
    Icon::new()
      .with_link("https://ziglang.org/",)
      .with_tooltip("General-purpose programming language and toolchain",)
      .with_label("Zig",)
  }

  /// Local SVG asset with `color-invert` applied for dark mode compatibility.
  pub fn local() -> Icon {
    base()
      .via_local("icons/logos/zig.svg",)
      .and_class("color-invert",)
  }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled [`icon::SiZig`] with `--brand-zig` colour.
  pub fn filled() -> Icon { base().via_leptos(icon::SiZig,).colored("brand-zig",) }
}
