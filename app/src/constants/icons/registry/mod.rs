mod data;
mod devops;
mod languages;
mod social;
mod tools;
mod ui;
mod web;

pub use {
  data::*,
  devops::*,
  languages::*,
  social::*,
  tools::*,
  ui::*,
  web::*,
};

pub mod _prelude {
  pub use {
    super::super::{
      Source,
      Variant,
    },
    crate::prelude::{
      Icon,
      icon,
    },
  };
}

pub mod prelude {
  pub use super::from_tag as icon_from_tag;
}

use _prelude::*;

// -- Tag → Icon ────────────────────────────────────────────────────────────────
type TagEntry = (&'static [&'static str], fn() -> Icon,);
const TAG_MAP : &[TagEntry] = &[
  // Languages
  (bash::TAGS, bash::default,),
  (javascript::TAGS, javascript::default,),
  (nushell::TAGS, nushell::default,),
  (posix::TAGS, posix::default,),
  (powershell::TAGS, powershell::default,),
  (python::TAGS, python::default,),
  (rust::TAGS, rust::default,),
  (typescript::TAGS, typescript::default,),
  (zig::TAGS, zig::default,),
  // Frameworks & Web
  (actix::TAGS, actix::default,),
  (axum::TAGS, axum::default,),
  (htmx::TAGS, htmx::default,),
  (leptos::TAGS, leptos::default,),
  (tailwind::TAGS, tailwind::default,),
  // Databases
  (deltalake::TAGS, deltalake::default,),
  (neo4j::TAGS, neo4j::default,),
  (postgresql::TAGS, postgresql::default,),
  (sqlite::TAGS, sqlite::default,),
  (surrealdb::TAGS, surrealdb::default,),
  // DevOps & Infrastructure
  (ansible::TAGS, ansible::default,),
  (docker::TAGS, docker::default,),
  (git::TAGS, git::default,),
  (github::TAGS, github::default,),
  (gitlab::TAGS, gitlab::default,),
  (kubernetes::TAGS, kubernetes::default,),
  (linux::TAGS, linux::default,),
  (nix::TAGS, nix::default,),
  (raspberry_pi::TAGS, raspberry_pi::default,),
  (terraform::TAGS, terraform::default,),
  (windows::TAGS, windows::default,),
  // Editors & Tools
  (helix::TAGS, helix::default,),
  (ohmyposh::TAGS, ohmyposh::default,),
  (starship::TAGS, starship::default,),
  (typst::TAGS, typst::default,),
  (vscode::TAGS, vscode::default,),
  (vscode_insiders::TAGS, vscode_insiders::default,),
  (vscodium::TAGS, vscodium::default,),
  (zed::TAGS, zed::default,),
  // Social & Messaging
  (facebook::TAGS, facebook::default,),
  (gmail::TAGS, gmail::default,),
  (instagram::TAGS, instagram::default,),
  (linkedin::TAGS, linkedin::default,),
  (whatsapp::TAGS, whatsapp::default,),
  (x::TAGS, x::default,),
  // UI & Navigation
  (theme_light::TAGS, theme_light::default,),
  (theme_dark::TAGS, theme_dark::default,),
  (theme_system::TAGS, theme_system::default,),
  (menu_open::TAGS, menu_open::default,),
  (menu_close::TAGS, menu_close::default,),
];

#[must_use]
pub fn from_tag(tag : &str,) -> Option<Icon,> {
  TAG_MAP
    .iter()
    .find(|(tags, _,)| tags.contains(&tag,),)
    .map(|(_, f,)| f(),)
}
