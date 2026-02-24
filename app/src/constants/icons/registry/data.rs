use crate::prelude::icons::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Delta Lake                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub fn deltalake() -> Icon {
  Icon::new_local("icons/logos/deltalake.svg",)
    .with_link("https://delta.io/",)
    .with_tooltip("Open-source storage framework",)
    .with_label("Delta Lake",)
}

pub mod deltalake_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/deltalake.svg",) }
  pub fn filled() -> Source { local() }
  pub fn outlined() -> Source { local() }

  pub fn with_color(source : Source,) -> Icon { deltalake().with_source(source,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SurrealDB                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub fn surrealdb() -> Icon {
  Icon::new_leptos(icon::SiSurrealdb,)
    .with_link("https://surrealdb.com/",)
    .with_tooltip("Multi-model database",)
    .with_label("SurrealDB",)
}

pub mod surrealdb_variants {
  use super::*;

  pub fn filled() -> Source { Source::Leptos(icon::SiSurrealdb,) }
  pub fn outlined() -> Source { filled() }

  pub fn with_color(source : Source,) -> Icon { surrealdb().with_source(source,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Neo4j                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub fn neo4j() -> Icon {
  Icon::new_local("icons/logos/neo4j-flat.svg",)
    .with_link("https://neo4j.com/",)
    .with_tooltip("Graph database platform",)
    .with_label("Neo4j",)
}

pub mod neo4j_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/neo4j-flat.svg",) }
  pub fn filled() -> Source { local() }
  pub fn outlined() -> Source { local() }

  pub fn with_color(source : Source,) -> Icon { neo4j().with_source(source,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PostgreSQL                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub fn postgresql() -> Icon {
  Icon::new_local("icons/logos/postgresql.svg",)
    .with_link("https://www.postgresql.org/",)
    .with_tooltip("Advanced open source relational database",)
    .with_label("PostgreSQL",)
}

pub mod postgresql_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/postgresql.svg",) }
  pub fn filled() -> Source { Source::Leptos(icon::SiPostgresql,) }
  pub fn outlined() -> Source { filled() }

  pub fn with_color(source : Source,) -> Icon { postgresql().with_source(source,) }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SQLite                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub fn sqlite() -> Icon {
  Icon::new_local("icons/logos/SQLite.svg",)
    .with_link("https://www.sqlite.org/",)
    .with_tooltip("Self-contained embedded database",)
    .with_label("SQLite",)
}

pub mod sqlite_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/SQLite.svg",) }
  pub fn filled() -> Source { Source::Leptos(icon::SiSqlite,) }
  pub fn outlined() -> Source { filled() }

  pub fn with_color(source : Source,) -> Icon { sqlite().with_source(source,) }
}
