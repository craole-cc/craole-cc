//! Central icon registry — one source of truth for every icon in the project.

use super::{Variant};icondata as ico;

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
}

impl Icons {
  pub fn variant(self) -> Variant {
    match self {
      // Leptos icons — brand colours applied to both themes
      Self::Rust   => Variant::leptos(ico::FaRustBrands)
                        .both_class("fill-[#D34516] dark:fill-[#F4A07C]"),
      Self::GitHub => Variant::leptos(ico::FaGithubBrands)
                        .both_class("fill-current dark:fill-white"),

      // Leptos icons — dark invert only
      Self::Actix  => Variant::leptos(ico::SiActix).dark_class(DARK),
      Self::Htmx   => Variant::leptos(ico::SiHtmx).dark_class(DARK),

      // Local assets — no theme adjustment needed
      Self::Python     => Variant::local("icons/logos/python.svg"),
      Self::Tailwind   => Variant::local("icons/logos/tailwind-blue.svg"),
      Self::Leptos     => Variant::local("icons/logos/leptos.ico"),
      Self::SurrealDb  => Variant::local("icons/logos/surrealdb.png"),
      Self::Neo4j      => Variant::local("icons/logos/neo4j-flat.svg"),
      Self::PostgreSql => Variant::local("icons/logos/postgresql.svg"),
      Self::Git        => Variant::local("icons/logos/git.svg"),

      // Local assets — dark invert
      Self::Axum        => Variant::local("icons/logos/tokio.svg").dark_class(DARK),
      Self::Zig         => Variant::local("icons/logos/zig.svg").dark_class(DARK),
      Self::Shell       => Variant::local("icons/logos/bash.svg").dark_class(DARK),
      Self::Bash        => Variant::local("icons/logos/bash.svg").dark_class(DARK),
      Self::DeltaLake   => Variant::local("icons/logos/deltalake.svg").dark_class(DARK),
      Self::Sqlite      => Variant::local("icons/logos/SQLite.svg").dark_class(DARK),
      Self::NixOs       => Variant::local("icons/logos/nix.svg").dark_class(DARK),
      Self::RaspberryPi => Variant::local("icons/logos/raspberry.svg").dark_class(DARK),
      Self::Windows     => Variant::local("icons/logos/windows.svg").dark_class(DARK),
      Self::Helix       => Variant::local("icons/logos/helix.svg").dark_class(DARK),
      Self::Typst       => Variant::local("icons/logos/typst.svg").dark_class(DARK),
      Self::VSCode      => Variant::local("icons/logos/vscode.svg").dark_class(DARK),
      Self::Zed         => Variant::local("icons/logos/zed.svg").dark_class(DARK),
      Self::PowerShell  => Variant::local("icons/logos/powershell.svg").dark_class(DARK),
      Self::Starship    => Variant::local("icons/logos/starship.svg").dark_class(DARK),
      Self::OhMyPosh    => Variant::local("icons/logos/ohmyposh.svg").dark_class(DARK),
    }
  }
}
