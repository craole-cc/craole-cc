use {
  super::{
    devops,
    social,
  },
  crate::prelude::Icon,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
  pub fn get(self) -> Icon {
    use Icons::*;
    match self {
      // Rust => languages::rust(),
      // Shell => languages::shell(),
      // Python => languages::python(),
      // Zig => languages::zig(),
      // Bash => languages::bash(),

      // Actix => web::actix(),
      // Axum => web::axum(),
      // Htmx => web::htmx(),
      // Leptos => web::leptos(),
      // Tailwind => web::tailwind(),

      // DeltaLake => data::deltalake(),
      // SurrealDb => data::surrealdb(),
      // Neo4j => data::neo4j(),
      // PostgreSql => data::postgresql(),
      // Sqlite => data::sqlite(),

      // Git => devops::git(),
      GitHub => devops::github(),
      // NixOs => devops::nixos(),
      // RaspberryPi => devops::raspberry_pi(),
      // Windows => devops::windows(),

      // Helix => editors::helix(),
      // Typst => editors::typst(),
      // VSCode => editors::vscode(),
      // Zed => editors::zed(),

      // PowerShell => terminal::powershell(),
      // Starship => terminal::starship(),
      // OhMyPosh => terminal::ohmyposh(),
      Gmail => social::gmail(),
      LinkedIn => social::linkedin(),
      WhatsApp => social::whatsapp(),
      Instagram => social::instagram(),
      Facebook => social::facebook(),
      X => social::x(),

      // Temporary stub for disabled categories
      _ => Icon::new(),
    }
  }
}
