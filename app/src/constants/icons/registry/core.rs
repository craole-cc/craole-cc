use crate::prelude::{
  icons::{
    registry::{
      data,
      devops,
      social,
    },
    *,
  },
  *,
};

const DARK_INVERT : &str = "dark:invert dark:hue-rotate-180";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,)]
pub enum Icons {
  // Languages & Core
  Rust,
  Shell,
  Python,
  Zig,
  Bash,

  // Web
  Actix,
  Axum,
  Htmx,
  Leptos,
  Tailwind,

  // Data
  DeltaLake,
  SurrealDb,
  Neo4j,
  PostgreSql,
  Sqlite,

  // DevOps & Systems
  Git,
  GitHub,
  NixOs,
  RaspberryPi,
  Windows,

  // Editors
  Helix,
  Typst,
  VSCode,
  Zed,

  // Terminal
  PowerShell,
  Starship,
  OhMyPosh,

  // Social Media
  Gmail,
  LinkedIn,
  WhatsApp,
  Instagram,
  Facebook,
  X,
}

impl Icons {
  pub fn get(self,) -> Icon {
    use Icons::*;
    match self {
      //╔═══════════════════════════════════════════════════════════╗
      //║ Languages                                                 ║
      //╚═══════════════════════════════════════════════════════════╝
      | Rust => Icon::new_leptos(icon::FaRustBrands,)
        .with_class("fill-[#D34516] dark:fill-[#F4A07C]",)
        .with_tooltip("Rust programming language",)
        .with_label("Rust",),

      | Shell | Bash => Icon::new_local("icons/logos/bash.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Shell",),

      | Python => Icon::new_local("icons/logos/python.svg",).with_label("Python",),

      | Zig => Icon::new_local("icons/logos/zig.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Zig",),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Web                                                       ║
      //╚═══════════════════════════════════════════════════════════╝
      | Actix => Icon::new_leptos(icon::SiActix,)
        .with_class(DARK_INVERT,)
        .with_label("Actix",),

      | Axum => Icon::new_local("icons/logos/tokio.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Axum",),

      | Htmx => Icon::new_leptos(icon::SiHtmx,)
        .with_class(DARK_INVERT,)
        .with_label("HTMX",),

      | Leptos => Icon::new_local("icons/logos/leptos.ico",).with_label("Leptos",),

      | Tailwind => Icon::new_local("icons/logos/tailwind-blue.svg",).with_label("Tailwind CSS",),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Data                                                      ║
      //╚═══════════════════════════════════════════════════════════╝
      | DeltaLake => data::deltalake(),
      | SurrealDb => data::surrealdb(),
      | Neo4j => data::neo4j(),
      | PostgreSql => data::postgresql(),
      | Sqlite => data::sqlite(),

      //╔═══════════════════════════════════════════════════════════╗
      //║ DevOps                                                    ║
      //╚═══════════════════════════════════════════════════════════╝
      | Git => devops::git(),
      | GitHub => devops::github(),
      | NixOs => devops::nix(),
      | RaspberryPi => devops::raspberry_pi(),
      | Windows => devops::windows(),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Editors                                                   ║
      //╚═══════════════════════════════════════════════════════════╝
      | Helix => Icon::new_local("icons/logos/helix.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Helix",),

      | Typst => Icon::new_local("icons/logos/typst.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Typst",),

      | VSCode => Icon::new_local("icons/logos/vscode.svg",)
        .with_class(DARK_INVERT,)
        .with_label("VS Code",),

      | Zed => Icon::new_local("icons/logos/zed.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Zed",),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Terminal                                                  ║
      //╚═══════════════════════════════════════════════════════════╝
      | PowerShell => Icon::new_local("icons/logos/powershell.svg",)
        .with_class(DARK_INVERT,)
        .with_label("PowerShell",),

      | Starship => Icon::new_local("icons/logos/starship.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Starship",),

      | OhMyPosh => Icon::new_local("icons/logos/ohmyposh.svg",)
        .with_class(DARK_INVERT,)
        .with_label("Oh My Posh",),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Social                                                    ║
      //╚═══════════════════════════════════════════════════════════╝
      | Gmail => social::gmail(),
      | LinkedIn => social::linkedin(),
      | WhatsApp => social::whatsapp(),
      | Instagram => social::instagram(),
      | Facebook => social::facebook(),
      | X => social::x(),
    }
  }
}
