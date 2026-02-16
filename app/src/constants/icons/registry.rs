//! Central icon registry — one source of truth for every icon in the project.

use super::{
  Variant,
  ico,
};

const DARK: &str = "dark:invert dark:hue-rotate-180";

/// Every icon used in the application.
///
/// Call `.variant()` to get the resolved light/dark [`Variant`].
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
  pub fn variant(self) -> Variant {
    match self {
      // ── Tech Stack ──────────────────────────────────────────────────────

      // Leptos icons — brand colours
      Self::Rust => Variant::leptos(ico::FaRustBrands)
        .both_class("fill-[#D34516] dark:fill-[#F4A07C]")
        .with_label("Rust")
        .with_tooltip("Rust programming language"),

      Self::GitHub => Variant::leptos(ico::FaGithubBrands)
        .both_class("fill-current dark:fill-white")
        .with_label("GitHub")
        .with_link("https://github.com/craole-cc")
        .with_tooltip("View my GitHub profile"),

      // Leptos icons — dark invert
      Self::Actix => Variant::leptos(ico::SiActix)
        .dark_class(DARK)
        .with_label("Actix"),

      Self::Htmx => Variant::leptos(ico::SiHtmx)
        .dark_class(DARK)
        .with_label("HTMX"),

      // Local assets — no theme adjustment
      Self::Python => Variant::local("icons/logos/python.svg").with_label("Python"),
      Self::Tailwind => Variant::local("icons/logos/tailwind-blue.svg").with_label("Tailwind CSS"),
      Self::Leptos => Variant::local("icons/logos/leptos.ico").with_label("Leptos"),
      Self::SurrealDb => Variant::local("icons/logos/surrealdb.png").with_label("SurrealDB"),
      Self::Neo4j => Variant::local("icons/logos/neo4j-flat.svg").with_label("Neo4j"),
      Self::PostgreSql => Variant::local("icons/logos/postgresql.svg").with_label("PostgreSQL"),
      Self::Git => Variant::local("icons/logos/git.svg").with_label("Git"),

      // Local assets — dark invert
      Self::Axum => Variant::local("icons/logos/tokio.svg")
        .dark_class(DARK)
        .with_label("Axum"),

      Self::Zig => Variant::local("icons/logos/zig.svg")
        .dark_class(DARK)
        .with_label("Zig"),

      Self::Shell => Variant::local("icons/logos/bash.svg")
        .dark_class(DARK)
        .with_label("Shell"),

      Self::Bash => Variant::local("icons/logos/bash.svg")
        .dark_class(DARK)
        .with_label("Bash"),

      Self::DeltaLake => Variant::local("icons/logos/deltalake.svg")
        .dark_class(DARK)
        .with_label("Delta Lake"),

      Self::Sqlite => Variant::local("icons/logos/SQLite.svg")
        .dark_class(DARK)
        .with_label("SQLite"),

      Self::NixOs => Variant::local("icons/logos/nix.svg")
        .dark_class(DARK)
        .with_label("NixOS"),

      Self::RaspberryPi => Variant::local("icons/logos/raspberry.svg")
        .dark_class(DARK)
        .with_label("Raspberry Pi"),

      Self::Windows => Variant::local("icons/logos/windows.svg")
        .dark_class(DARK)
        .with_label("Windows"),

      Self::Helix => Variant::local("icons/logos/helix.svg")
        .dark_class(DARK)
        .with_label("Helix"),

      Self::Typst => Variant::local("icons/logos/typst.svg")
        .dark_class(DARK)
        .with_label("Typst"),

      Self::VSCode => Variant::local("icons/logos/vscode.svg")
        .dark_class(DARK)
        .with_label("VS Code"),

      Self::Zed => Variant::local("icons/logos/zed.svg")
        .dark_class(DARK)
        .with_label("Zed"),

      Self::PowerShell => Variant::local("icons/logos/powershell.svg")
        .dark_class(DARK)
        .with_label("PowerShell"),

      Self::Starship => Variant::local("icons/logos/starship.svg")
        .dark_class(DARK)
        .with_label("Starship"),

      Self::OhMyPosh => Variant::local("icons/logos/ohmyposh.svg")
        .dark_class(DARK)
        .with_label("Oh My Posh"),

      // ── Social Media ────────────────────────────────────────────────────
      Self::Gmail => Variant::local("icons/logos/gmail.svg")
        .with_label("Gmail")
        .with_link("mailto:craig.craole.cole@gmail.com")
        .with_tooltip("Send me an email"),

      Self::LinkedIn => Variant::local("icons/logos/linkedin.svg")
        .with_label("LinkedIn")
        .with_link("https://linkedin.com/in/craole")
        .with_tooltip("Connect on LinkedIn"),

      Self::WhatsApp => Variant::local("icons/logos/whatsapp.svg")
        .with_label("WhatsApp")
        .with_link("https://wa.me/18768130049")
        .with_tooltip("Message me on WhatsApp"),

      Self::Instagram => Variant::local("icons/logos/instagram.svg")
        .with_label("Instagram")
        .with_link("https://instagram.com/craole")
        .with_tooltip("Follow me on Instagram"),

      Self::Facebook => Variant::local("icons/logos/facebook.svg")
        .with_label("Facebook")
        .with_link("https://facebook.com/craole")
        .with_tooltip("Connect on Facebook"),

      Self::X => Variant::local("icons/logos/x.svg")
        .with_label("X")
        .with_link("https://x.com/craole")
        .with_tooltip("Follow me on X"),
    }
  }
}
