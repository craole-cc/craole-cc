use crate::prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ JavaScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod javascript {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.javascript.com/",)
      .with_tooltip("Lightweight, interpreted programming language for the web",)
      .with_label("JavaScript",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_JS_YELLOW, CLR_JS_YELLOW,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/javascript.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiJavascript,) }
  pub fn outlined() -> Icon { leptos(icon::TbBrandJavascriptOutline,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ TypeScript                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod typescript {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.typescriptlang.org/",)
      .with_tooltip("Strongly typed superset of JavaScript",)
      .with_label("TypeScript",)
  }

  fn leptos(src : IconData,) -> Icon { base().via_leptos(src,).colorize(CLR_TS_BLUE, CLR_TS_BLUE,) }

  pub fn local() -> Icon { base().via_local("icons/logos/typescript.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiTypescript,) }
  pub fn outlined() -> Icon { leptos(icon::TbBrandTypescriptOutline,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Nushell                                                   ║
//╚═══════════════════════════════════════════════════════════╝
pub mod nushell {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.nushell.sh",)
      .with_tooltip("A modern shell and scripting language",)
      .with_label("Nushell",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_NUSHELL_LIGHT, CLR_NUSHELL_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/nushell.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiNushell,) }
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PowerShell                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod powershell {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://learn.microsoft.com/en-us/powershell/",)
      .with_tooltip("Task automation and configuration management framework",)
      .with_label("PowerShell",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_POWERSHELL_LIGHT, CLR_POWERSHELL_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/powershell.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::MdiPowershell,) }
  pub fn outlined() -> Icon { leptos(icon::TbBrandPowershellOutline,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Python                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod python {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.python.org/",)
      .with_tooltip("Versatile, high-level programming language",)
      .with_label("Python",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_PYTHON_LIGHT, CLR_PYTHON_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/python.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiPython,) }
  pub fn outlined() -> Icon { leptos(icon::AiPythonOutlined,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Rust                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod rust {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.rust-lang.org/",)
      .with_tooltip("Systems programming language focused on safety and performance",)
      .with_label("Rust",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_RUST_LIGHT, CLR_RUST_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/rust.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiRust,) }
  pub fn outlined() -> Icon { leptos(icon::TbBrandRustOutline,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Bash                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod bash {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://www.gnu.org/software/bash/",)
      .with_tooltip("Bourne Again Shell - Unix shell and command language",)
      .with_label("Bash",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_BASH_LIGHT, CLR_BASH_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/bash.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiGnubash,) }
  pub fn outlined() -> Icon { leptos(icon::MdiBash,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ POSIX Shell                                               ║
//╚═══════════════════════════════════════════════════════════╝
pub mod posix {
  use super::*;

  fn base() -> Icon {
    Icon::new()
      .with_link("https://pubs.opengroup.org/onlinepubs/9799919799/",)
      .with_tooltip("Portable Operating System Interface shell standard",)
      .with_label("POSIX Shell",)
  }

  fn leptos(src : IconData,) -> Icon {
    base()
      .via_leptos(src,)
      .colorize(CLR_POSIX_LIGHT, CLR_POSIX_DARK,)
  }

  pub fn local() -> Icon { base().via_local("icons/logos/posix-shell.svg",) }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { leptos(icon::SiGnubash,) }
  pub fn outlined() -> Icon { leptos(icon::MdiBash,) }
}
