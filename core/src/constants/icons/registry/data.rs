use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Delta Lake                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod deltalake {
  use super::{
    Icon,
    Variant,
  };

  pub struct DeltaLake(pub Variant,);

  impl DeltaLake {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/deltalake.svg",)
      .with_link("https://delta.io/",)
      .with_tooltip("Open-source storage framework",)
      .with_label("Delta Lake",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon { local() }

  #[must_use]
  pub fn outlined() -> Icon { local() }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Neo4j                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod neo4j {
  use super::{
    Icon,
    Variant,
  };

  pub struct Neo4j(pub Variant,);

  impl Neo4j {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }
  const fn base() -> Icon {
    Icon::new()
      .with_link("https://neo4j.com/",)
      .with_tooltip("Graph database platform",)
      .with_label("Neo4j",)
  }

  #[must_use]
  pub const fn local() -> Icon { base().via_local("icons/logos/neo4j.svg",) }

  #[must_use]
  pub const fn filled() -> Icon { local() }

  #[must_use]
  pub const fn outlined() -> Icon { local() }

  #[must_use]
  pub const fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PostgreSQL                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod postgresql {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct PostgreSQL(pub Variant,);

  impl PostgreSQL {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/postgresql.svg",)
      .with_link("https://www.postgresql.org/",)
      .with_tooltip("Advanced open source relational database",)
      .with_label("PostgreSQL",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiPostgresql,)
      .colored("brand-postgresql",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SQLite                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod sqlite {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct SQLite(pub Variant,);

  impl SQLite {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/SQLite.svg",)
      .with_link("https://www.sqlite.org/",)
      .with_tooltip("Self-contained embedded database",)
      .with_label("SQLite",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon { base().via_leptos(icon::SiSqlite,).colored("brand-sqlite",) }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SurrealDB                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod surrealdb {
  use super::{
    Icon,
    Variant,
    icon,
  };

  pub struct SurrealDB(pub Variant,);

  impl SurrealDB {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/surrealdb.svg",)
      .with_link("https://surrealdb.com/",)
      .with_tooltip("Multi-model database",)
      .with_label("SurrealDB",)
  }

  #[must_use]
  pub fn local() -> Icon { base() }

  #[must_use]
  pub fn filled() -> Icon {
    base()
      .via_leptos(icon::SiSurrealdb,)
      .colored("brand-surrealdb",)
  }

  #[must_use]
  pub fn outlined() -> Icon { filled() }

  #[must_use]
  pub fn default() -> Icon { local() }
}
