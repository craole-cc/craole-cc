use crate::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Helix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod helix {
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
    Icon::new_local("icons/logos/helix.svg",)
      .with_link("https://helix-editor.com/",)
      .with_tooltip("Post-modern modal text editor",)
      .with_label("Helix",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Oh My Posh                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ohmyposh {
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
    Icon::new_local("icons/logos/ohmyposh.svg",)
      .with_link("https://ohmyposh.dev/",)
      .with_tooltip("Prompt theme engine for any shell",)
      .with_label("Oh My Posh",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Oh My Zsh                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ohmyzsh {
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
    Icon::new_local("icons/logos/ohmyzsh.svg",)
      .with_link("https://ohmyz.sh/",)
      .with_tooltip("Framework for managing Zsh configuration",)
      .with_label("Oh My Zsh",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Starship                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod starship {
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
    Icon::new_local("icons/logos/starship.svg",)
      .with_link("https://starship.rs/",)
      .with_tooltip("Minimal, fast, cross-shell prompt",)
      .with_label("Starship",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Typst                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typst {
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
    Icon::new_local("icons/logos/typst.svg",)
      .with_link("https://typst.app/",)
      .with_tooltip("Modern typesetting system",)
      .with_label("Typst",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode {
  use super::*;

  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/vscode.svg",)
      .with_link("https://code.visualstudio.com/",)
      .with_tooltip("Visual Studio Code editor",)
      .with_label("VS Code",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled VS Code icon with `--brand-vscode` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::VsVscode,),)
      .colored("brand-vscode",)
  }
  /// Outlined Tabler icon with `--brand-vscode` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::TbBrandVscodeOutline,),)
      .colored("brand-vscode",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code Insiders                                          ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode_insiders {
  use super::*;

  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
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

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled VS Code Insiders icon with `--brand-vscodeinsiders` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::VsVscodeInsiders,),)
      .colored("brand-vscodeinsiders",)
  }
  /// Outlined Tabler icon with `--brand-vscodeinsiders` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::TbBrandVscodeOutline,),)
      .colored("brand-vscodeinsiders",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VSCodium                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscodium {
  use super::*;

  impl From<Variant,> for Icon {
    fn from(v : Variant,) -> Icon {
      match v {
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

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled Simple Icons icon with `--brand-vscodium` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::SiVscodium,),)
      .colored("brand-vscodium",)
  }
  /// Outlined VS Code OSS icon with `--brand-vscodium` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::VsCodeOss,),)
      .colored("brand-vscodium",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Zed                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod zed {
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
    Icon::new_local("icons/logos/zed.svg",)
      .with_link("https://zed.dev/",)
      .with_tooltip("High-performance multiplayer code editor",)
      .with_label("Zed",)
  }

  /// Local SVG asset with `color-invert` for dark mode compatibility.
  pub fn local() -> Icon { base().and_class("color-invert",) }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
}
