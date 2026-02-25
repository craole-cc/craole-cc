mod data;
mod devops;
mod languages;
mod ui;
// mod editors;
mod social;
// mod terminal;
mod web;

use {
  crate::_prelude::*,
  data::*,
  devops::*,
  languages::*,
  social::*,
  ui::*,
  web::*,
};

const DARK_INVERT : &str = "dark:invert dark:hue-rotate-180";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,)]
pub enum Icons {
  //╔═══════════════════════════════════════════════════════════╗
  //║ UI                                                        ║
  //╚═══════════════════════════════════════════════════════════╝
  // Theme switcher icons — sourced from registry/ui.rs.
  // ThemeLight/ThemeDark drive the toggle button (always sun or moon).
  // ThemeSystem appears in the dropdown only, labelled "System".
  ThemeLight,
  ThemeDark,
  ThemeSystem,

  // Hamburger — shown in NavControls based on `open` signal.
  // MenuClosed = drawer is closed, button invites user to open it.
  // MenuOpen   = drawer is open, button invites user to close it.
  MenuClosed,
  MenuOpen,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Languages & Core                                          ║
  //╚═══════════════════════════════════════════════════════════╝
  Rust,
  Shell,
  Python,
  Zig,
  Bash,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Web                                                       ║
  //╚═══════════════════════════════════════════════════════════╝
  Actix,
  Axum,
  Htmx,
  Leptos,
  Tailwind,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Data                                                      ║
  //╚═══════════════════════════════════════════════════════════╝
  DeltaLake,
  SurrealDb,
  Neo4j,
  PostgreSql,
  Sqlite,

  //╔═══════════════════════════════════════════════════════════╗
  //║ DevOps & Systems                                          ║
  //╚═══════════════════════════════════════════════════════════╝
  Git,
  GitHub,
  NixOs,
  RaspberryPi,
  Windows,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Editors                                                   ║
  //╚═══════════════════════════════════════════════════════════╝
  Helix,
  Typst,
  VSCode,
  Zed,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Terminal                                                  ║
  //╚═══════════════════════════════════════════════════════════╝
  PowerShell,
  Starship,
  OhMyPosh,

  //╔═══════════════════════════════════════════════════════════╗
  //║ Social Media                                              ║
  //╚═══════════════════════════════════════════════════════════╝
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
      //║ UI                                            ║
      //╚═══════════════════════════════════════════════════════════╝
      | ThemeLight => theme_light::default(),
      | ThemeDark => theme_dark::default(),
      | ThemeSystem => theme_system::default(),
      | MenuClosed => menu_closed::default(),
      | MenuOpen => menu_open::default(),

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
      //║ Web Development                                           ║
      //╚═══════════════════════════════════════════════════════════╝
      | Actix => actix::default(),
      | Axum => axum::default(),
      | Htmx => htmx::default(),
      | Leptos => leptos::default(),
      | Tailwind => tailwind::default(),

      //╔═══════════════════════════════════════════════════════════╗
      //║ Data Engineering                                          ║
      //╚═══════════════════════════════════════════════════════════╝
      | DeltaLake => deltalake::default(),
      | SurrealDb => surrealdb::default(),
      | Neo4j => neo4j::default(),
      | PostgreSql => postgresql::default(),
      | Sqlite => sqlite::default(),

      //╔═══════════════════════════════════════════════════════════╗
      //║ DevOps                                                    ║
      //╚═══════════════════════════════════════════════════════════╝
      | Git => git::default(),
      | GitHub => github::default(),
      | NixOs => nix::default(),
      | RaspberryPi => raspberry_pi::default(),
      | Windows => windows::default(),

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
      | Facebook => facebook::default(),
      | Gmail => gmail::default(),
      | Instagram => instagram::default(),
      | LinkedIn => linkedin::default(),
      | WhatsApp => whatsapp::default(),
      | X => x::default(),
    }
  }
}
