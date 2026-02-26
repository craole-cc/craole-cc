use crate::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Delta Lake                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod deltalake {

  use super::*;

  /// No Leptos icon available yet — all variants resolve to the local asset.
  impl From<IconVariant,> for Icon {
    fn from(v : IconVariant,) -> Icon {
      match v {
        | IconVariant::Default => local(),
        | IconVariant::Local => local(),
        | IconVariant::Filled => local(),   // no upstream icon yet
        | IconVariant::Outlined => local(), // no upstream icon yet
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/deltalake.svg",)
      .with_link("https://delta.io/",)
      .with_tooltip("Open-source storage framework",)
      .with_label("Delta Lake",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  /// No Leptos icon available upstream — swap `filled` when one lands.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Falls back to local until an upstream Leptos icon is available.
  pub fn filled() -> Icon { local() }
  /// Falls back to local until an upstream Leptos icon is available.
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Neo4j                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod neo4j {
  use super::*;

  /// No Leptos icon available yet — all variants resolve to the local asset.
  impl From<IconVariant,> for Icon {
    fn from(v : IconVariant,) -> Icon {
      match v {
        | IconVariant::Default => local(),
        | IconVariant::Local => local(),
        | IconVariant::Filled => local(),   // no upstream icon yet
        | IconVariant::Outlined => local(), // no upstream icon yet
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/neo4j-flat.svg",)
      .with_link("https://neo4j.com/",)
      .with_tooltip("Graph database platform",)
      .with_label("Neo4j",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  /// No Leptos icon available upstream — swap `filled` when one lands.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Falls back to local until an upstream Leptos icon is available.
  pub fn filled() -> Icon { local() }
  /// Falls back to local until an upstream Leptos icon is available.
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PostgreSQL                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod postgresql {
  use super::*;

  /// No distinct outlined style — `Outlined` falls back to `Filled`.
  impl From<IconVariant,> for Icon {
    fn from(v : IconVariant,) -> Icon {
      match v {
        | IconVariant::Default => local(),
        | IconVariant::Local => local(),
        | IconVariant::Filled => filled(),
        | IconVariant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/postgresql.svg",)
      .with_link("https://www.postgresql.org/",)
      .with_tooltip("Advanced open source relational database",)
      .with_label("PostgreSQL",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled Simple Icons icon with `--brand-postgresql` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::SiPostgresql,),)
      .colored("brand-postgresql",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SQLite                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod sqlite {
  use super::*;

  /// No distinct outlined style — `Outlined` falls back to `Filled`.
  impl From<IconVariant,> for Icon {
    fn from(v : IconVariant,) -> Icon {
      match v {
        | IconVariant::Default => local(),
        | IconVariant::Local => local(),
        | IconVariant::Filled => filled(),
        | IconVariant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/SQLite.svg",)
      .with_link("https://www.sqlite.org/",)
      .with_tooltip("Self-contained embedded database",)
      .with_label("SQLite",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — uses the local asset.
  pub fn default() -> Icon { local() }
  /// Filled Simple Icons icon with `--brand-sqlite` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(IconSource::Leptos(icon::SiSqlite,),)
      .colored("brand-sqlite",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SurrealDB                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod surrealdb {
  use super::*;

  /// No local asset — all variants resolve to the Leptos icon.
  impl From<IconVariant,> for Icon {
    fn from(v : IconVariant,) -> Icon {
      match v {
        | IconVariant::Default => default(),
        | IconVariant::Local => default(), // no local asset
        | IconVariant::Filled => default(),
        | IconVariant::Outlined => default(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_leptos(icon::SiSurrealdb,)
      .with_link("https://surrealdb.com/",)
      .with_tooltip("Multi-model database",)
      .with_label("SurrealDB",)
  }

  /// Canonical default — Leptos icon, no local asset available.
  pub fn default() -> Icon { base() }
  /// Filled Simple Icons icon (same as default for SurrealDB).
  pub fn filled() -> Icon { base() }
}
