use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Helix                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod helix {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Helix(pub Variant,);

  impl Helix {
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
      .with_link("https://helix-editor.com/",)
      .with_tooltip("Post-modern modal text editor",)
      .with_label("Helix",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/helix.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiHelix,).colored("brand-helix",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Oh My Posh                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod ohmyposh {
  use super::{
    Icon,
    Variant,
  };

  pub struct OhMyPosh(pub Variant,);

  impl OhMyPosh {
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
      .with_link("https://ohmyposh.dev/",)
      .with_tooltip("Prompt theme engine for any shell",)
      .with_label("Oh My Posh",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/ohmyposh.svg",) }

  #[must_use]
  pub const fn filled() -> Icon { local() }

  #[must_use]
  pub const fn outlined() -> Icon { local() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Starship                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod starship {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Starship(pub Variant,);

  impl Starship {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://starship.rs/",)
      .with_tooltip("Minimal, fast, cross-shell prompt",)
      .with_label("Starship",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/starship.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiStarship,)
      .colored("brand-starship",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Typst                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typst {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Typst(pub Variant,);

  impl Typst {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://typst.app/",)
      .with_tooltip("Modern typesetting system",)
      .with_label("Typst",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/typst.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiTypst,).colored("brand-typst",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct VsCode(pub Variant,);

  impl VsCode {
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
      .with_link("https://code.visualstudio.com/",)
      .with_tooltip("Visual Studio Code editor",)
      .with_label("VS Code",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/vscode.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::VsVscode,).colored("brand-vscode",) }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandVscodeOutline,)
      .colored("brand-vscode",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VS Code Insiders                                          ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscode_insiders {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct VsCodeInsiders(pub Variant,);

  impl VsCodeInsiders {
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
    Icon::new_local("icons/logos/vscodium.svg",)
      .with_link("https://code.visualstudio.com/insiders/",)
      .with_tooltip("Visual Studio Code Insiders build",)
      .with_label("VS Code Insiders",)
  }

  #[must_use]
  pub fn local() -> Icon { base().and_class("color-invert",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::VsVscodeInsiders,)
      .colored("brand-vscodeinsiders",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandVscodeOutline,)
      .colored("brand-vscodeinsiders",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ VSCodium                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod vscodium {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct VsCodium(pub Variant,);

  impl VsCodium {
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
    Icon::new_local("icons/logos/vscodium.svg",)
      .with_link("https://vscodium.com/",)
      .with_tooltip("Community-driven, freely-licensed VS Code build",)
      .with_label("VSCodium",)
  }

  #[must_use]
  pub fn local() -> Icon { base().and_class("color-invert",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiVscodium,)
      .colored("brand-vscodium",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::VsCodeOss,)
      .colored("brand-vscodium",)
  }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Zed                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod zed {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Zed(pub Variant,);

  impl Zed {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  const fn base() -> Icon {
    Icon::new()
      .with_link("https://zed.dev/",)
      .with_tooltip("High-performance multiplayer code editor",)
      .with_label("Zed",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/zed.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiZedindustries,)
      .colored("brand-zed",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}
