//! Central icon registry — one source of truth for every icon in the project.

use super::{Icon, icon};

const DARK_INVERT: &str = "dark:invert dark:hue-rotate-180";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icons {
  // ── Languages & Core ──────────────────────────────────────────────────────
  Rust,
  Shell,
  Python,
  Zig,

  // ── Web ───────────────────────────────────────────────────────────────────
  Actix,
  Axum,
  Htmx,
  Leptos,
  Tailwind,

  // ── Data ──────────────────────────────────────────────────────────────────
  DeltaLake,
  SurrealDb,
  Neo4j,
  PostgreSql,
  Sqlite,

  // ── DevOps & Systems ──────────────────────────────────────────────────────
  Git,
  GitHub,
  NixOs,
  RaspberryPi,
  Windows,

  // ── Editors ───────────────────────────────────────────────────────────────
  Helix,
  Typst,
  VSCode,
  Zed,

  // ── Terminal ──────────────────────────────────────────────────────────────
  Bash,
  PowerShell,
  Starship,
  OhMyPosh,

  // ── Social Media ──────────────────────────────────────────────────────────
  Gmail,
  LinkedIn,
  WhatsApp,
  Instagram,
  Facebook,
  X,
}

impl Icons {
  /// Get the icon with all metadata and theme-aware classes baked in.
  pub fn get(self) -> Icon {
    match self {
      // ── Tech Stack ──────────────────────────────────────────────────────
      | Self::Rust => Icon::new_leptos(icon::FaRustBrands)
        .with_class("fill-[#D34516] dark:fill-[#F4A07C]")
        .with_label("Rust")
        .with_tooltip("Rust programming language"),

      | Self::GitHub => Icon::new_leptos(icon::FaGithubBrands)
        .with_class("fill-current dark:fill-white")
        .with_label("GitHub")
        .with_link("https://github.com/craole-cc")
        .with_tooltip("View my GitHub profile"),

      | Self::Actix => Icon::new_leptos(icon::SiActix)
        .with_class(DARK_INVERT)
        .with_label("Actix"),

      | Self::Htmx => Icon::new_leptos(icon::SiHtmx)
        .with_class(DARK_INVERT)
        .with_label("HTMX"),

      // Local assets
      | Self::Python => Icon::new_local("icons/logos/python.svg").with_label("Python"),

      | Self::Tailwind => {
        Icon::new_local("icons/logos/tailwind-blue.svg").with_label("Tailwind CSS")
      }

      | Self::Leptos => Icon::new_local("icons/logos/leptos.ico").with_label("Leptos"),

      | Self::SurrealDb => Icon::new_local("icons/logos/surrealdb.png").with_label("SurrealDB"),

      | Self::Neo4j => Icon::new_local("icons/logos/neo4j-flat.svg").with_label("Neo4j"),

      | Self::PostgreSql => Icon::new_local("icons/logos/postgresql.svg").with_label("PostgreSQL"),

      | Self::Git => Icon::new_local("icons/logos/git.svg").with_label("Git"),

      | Self::Axum => Icon::new_local("icons/logos/tokio.svg")
        .with_class(DARK_INVERT)
        .with_label("Axum"),

      | Self::Zig => Icon::new_local("icons/logos/zig.svg")
        .with_class(DARK_INVERT)
        .with_label("Zig"),

      | Self::Shell => Icon::new_local("icons/logos/bash.svg")
        .with_class(DARK_INVERT)
        .with_label("Shell"),

      | Self::Bash => Icon::new_local("icons/logos/bash.svg")
        .with_class(DARK_INVERT)
        .with_label("Bash"),

      | Self::DeltaLake => Icon::new_local("icons/logos/deltalake.svg")
        .with_class(DARK_INVERT)
        .with_label("Delta Lake"),

      | Self::Sqlite => Icon::new_local("icons/logos/SQLite.svg")
        .with_class(DARK_INVERT)
        .with_label("SQLite"),

      | Self::NixOs => Icon::new_local("icons/logos/nix.svg")
        .with_class(DARK_INVERT)
        .with_label("NixOS"),

      | Self::RaspberryPi => Icon::new_local("icons/logos/raspberry.svg")
        .with_class(DARK_INVERT)
        .with_label("Raspberry Pi"),

      | Self::Windows => Icon::new_local("icons/logos/windows.svg")
        .with_class(DARK_INVERT)
        .with_label("Windows"),

      | Self::Helix => Icon::new_local("icons/logos/helix.svg")
        .with_class(DARK_INVERT)
        .with_label("Helix"),

      | Self::Typst => Icon::new_local("icons/logos/typst.svg")
        .with_class(DARK_INVERT)
        .with_label("Typst"),

      | Self::VSCode => Icon::new_local("icons/logos/vscode.svg")
        .with_class(DARK_INVERT)
        .with_label("VS Code"),

      | Self::Zed => Icon::new_local("icons/logos/zed.svg")
        .with_class(DARK_INVERT)
        .with_label("Zed"),

      | Self::PowerShell => Icon::new_local("icons/logos/powershell.svg")
        .with_class(DARK_INVERT)
        .with_label("PowerShell"),

      | Self::Starship => Icon::new_local("icons/logos/starship.svg")
        .with_class(DARK_INVERT)
        .with_label("Starship"),

      | Self::OhMyPosh => Icon::new_local("icons/logos/ohmyposh.svg")
        .with_class(DARK_INVERT)
        .with_label("Oh My Posh"),

      // ── Social Media ────────────────────────────────────────────────────
      | Self::Gmail => Icon::new_local("icons/logos/gmail.svg")
        .with_label("Gmail")
        .with_link("mailto:craig.craole.cole@gmail.com")
        .with_tooltip("Send me an email"),

      | Self::LinkedIn => Icon::new_local("icons/logos/linkedin.svg")
        .with_label("LinkedIn")
        .with_link("https://linkedin.com/in/craole")
        .with_tooltip("Connect on LinkedIn"),

      | Self::WhatsApp => Icon::new_local("icons/logos/whatsapp.svg")
        .with_label("WhatsApp")
        .with_link("https://wa.me/18768130049")
        .with_tooltip("Message me on WhatsApp"),

      | Self::Instagram => Icon::new_local("icons/logos/instagram.svg")
        .with_label("Instagram")
        .with_link("https://instagram.com/craole")
        .with_tooltip("Follow me on Instagram"),

      | Self::Facebook => Icon::new_local("icons/logos/facebook.svg")
        .with_label("Facebook")
        .with_link("https://facebook.com/craole")
        .with_tooltip("Connect on Facebook"),

      | Self::X => Icon::new_leptos(icon::AiXFilled)
        .with_label("X")
        .with_link("https://x.com/craole")
        .with_tooltip("Follow me on X"),
    }
  }
}
