use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Bash                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod bash {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Bash(pub Variant,);

  impl Bash {
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
      .with_link("https://www.gnu.org/software/bash/",)
      .with_tooltip("Bourne Again Shell - Unix shell and command language",)
      .with_label("Bash",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/bash.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiGnubash,).colored("brand-bash",) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::MdiBash,).colored("brand-bash",) }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ JavaScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod javascript {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct JavaScript(pub Variant,);

  impl JavaScript {
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
      .with_link("https://www.javascript.com/",)
      .with_tooltip("Lightweight, interpreted programming language for the web",)
      .with_label("JavaScript",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/javascript.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiJavascript,)
      .colored("brand-javascript",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandJavascriptOutline,)
      .colored("brand-javascript",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nushell                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nushell {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Nushell(pub Variant,);

  impl Nushell {
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
      .with_link("https://www.nushell.sh",)
      .with_tooltip("A modern shell and scripting language",)
      .with_label("Nushell",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/nushell.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiNushell,)
      .colored("brand-nushell",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ POSIX                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod posix {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Posix(pub Variant,);

  impl Posix {
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
      .with_link("https://pubs.opengroup.org/onlinepubs/9799919799/",)
      .with_tooltip("Portable Operating System Interface shell standard",)
      .with_label("POSIX Shell",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/posix-shell.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiGnubash,).colored("brand-posix",) }

  #[must_use]
  pub fn outlined() -> Icon { base().via_leptos(icon::MdiBash,).colored("brand-posix",) }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PowerShell                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod powershell {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct PowerShell(pub Variant,);

  impl PowerShell {
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
      .with_link("https://learn.microsoft.com/en-us/powershell/",)
      .with_tooltip("Task automation and configuration management framework",)
      .with_label("PowerShell",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/powershell.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::MdiPowershell,)
      .colored("brand-powershell",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandPowershellOutline,)
      .colored("brand-powershell",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Python                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod python {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Python(pub Variant,);

  impl Python {
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
      .with_link("https://www.python.org/",)
      .with_tooltip("Versatile, high-level programming language",)
      .with_label("Python",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/python.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiPython,).colored("brand-python",) }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::AiPythonOutlined,)
      .colored("brand-python",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Rust                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod rust {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Rust(pub Variant,);

  impl Rust {
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
      .with_link("https://www.rust-lang.org/",)
      .with_tooltip("Systems programming language focused on safety and performance",)
      .with_label("Rust",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/rust.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiRust,).colored("brand-rust",) }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandRustOutline,)
      .colored("brand-rust",)
  }

  #[must_use]
  pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ TypeScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typescript {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct TypeScript(pub Variant,);

  impl TypeScript {
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
      .with_link("https://www.typescriptlang.org/",)
      .with_tooltip("Strongly typed superset of JavaScript",)
      .with_label("TypeScript",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/typescript.svg",) }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiTypescript,)
      .colored("brand-typescript",)
  }

  #[must_use]
  pub fn outlined() -> Icon {
    base()
      .via_leptos(icon::TbBrandTypescriptOutline,)
      .colored("brand-typescript",)
  }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Zig                                                       ║
//╚═══════════════════════════════════════════════════════════╝
pub mod zig {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct Zig(pub Variant,);

  impl Zig {
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
      .with_link("https://ziglang.org/",)
      .with_tooltip("General-purpose programming language and toolchain",)
      .with_label("Zig",)
  }

  #[must_use]
  pub fn local() -> Icon { base().via_local("icons/logos/zig.svg",) }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiZig,).colored("brand-zig",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { filled() }
}
