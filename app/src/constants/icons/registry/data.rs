use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Delta Lake                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod deltalake {
  use super::*;

  /// Icon selector for [Delta Lake](https://delta.io/), an open-source
  /// storage framework built on Apache Parquet.
  ///
  /// Local-only — no upstream Leptos icon exists yet. All variants resolve
  /// to the bundled SVG. Swap [`filled`] for a Leptos icon when one lands.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`local`]   |
  /// | Outlined | [`local`]   |
  pub struct DeltaLake(pub Variant,);

  impl DeltaLake {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => local(),
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
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn filled() -> Icon { local() }
  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Neo4j                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod neo4j {
  use super::*;

  /// Icon selector for [Neo4j](https://neo4j.com/), a native graph
  /// database and analytics platform.
  ///
  /// Local-only — no upstream Leptos icon exists yet. All variants resolve
  /// to the bundled SVG. Swap [`filled`] for a Leptos icon when one lands.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`local`]   |
  /// | Outlined | [`local`]   |
  pub struct Neo4j(pub Variant,);

  impl Neo4j {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default | Variant::Local | Variant::Filled | Variant::Outlined => local(),
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
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn filled() -> Icon { local() }
  /// Falls back to [`local`] — no upstream Leptos icon yet.
  pub fn outlined() -> Icon { local() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ PostgreSQL                                                ║
//╚═══════════════════════════════════════════════════════════╝
pub mod postgresql {
  use super::*;

  /// Icon selector for [PostgreSQL](https://www.postgresql.org/), an
  /// advanced open-source relational database.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiPostgresql` with `--brand-postgresql` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = postgresql::PostgreSQL(Variant::Filled,).get(); 
  /// ```
  pub struct PostgreSQL(pub Variant,);

  impl PostgreSQL {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
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

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiPostgresql`](icon::SiPostgresql) with `--brand-postgresql` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiPostgresql,),)
      .colored("brand-postgresql",)
  }

  /// Falls back to [`filled`] — no distinct outlined style exists.
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SQLite                                                    ║
//╚═══════════════════════════════════════════════════════════╝
pub mod sqlite {
  use super::*;

  /// Icon selector for [SQLite](https://www.sqlite.org/), a
  /// self-contained, serverless embedded database engine.
  ///
  /// No distinct outlined style — [`Variant::Outlined`] falls back to
  /// [`filled`].
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`local`]   |
  /// | Local    | [`local`]   |
  /// | Filled   | [`filled`] — `SiSqlite` with `--brand-sqlite` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = sqlite::SQLite(Variant::Filled,).get(); 
  /// ```
  pub struct SQLite(pub Variant,);

  impl SQLite {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
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

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiSqlite`](icon::SiSqlite) with `--brand-sqlite` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiSqlite,),)
      .colored("brand-sqlite",)
  }

  /// Falls back to [`filled`] — no distinct outlined style exists.
  pub fn outlined() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ SurrealDB                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod surrealdb {
  use super::*;

  /// Icon selector for [SurrealDB](https://surrealdb.com/), a
  /// multi-model database supporting graph, document, and relational data.
  ///
  /// No local SVG asset — all variants resolve to the Leptos icon.
  /// `Default` resolves to [`filled`] since there is no monochrome local
  /// fallback.
  ///
  /// # Variants
  /// | Variant  | Resolves to |
  /// |----------|-------------|
  /// | Default  | [`filled`] — `SiSurrealdb` with `--brand-surrealdb` |
  /// | Local    | [`filled`] — no local asset available |
  /// | Filled   | [`filled`] — `SiSurrealdb` with `--brand-surrealdb` |
  /// | Outlined | [`filled`] — no distinct outlined style |
  ///
  /// # Example
  /// ```rust
  /// let icon = surrealdb::SurrealDB(Variant::Filled,).get(); 
  /// ```
  pub struct SurrealDB(pub Variant,);

  impl SurrealDB {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Outlined` falls back to [`filled`] — no distinct outlined style exists.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => filled(),
      }
    }
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/surrealdb.svg",) // ← now has a local asset
      .with_link("https://surrealdb.com/",)
      .with_tooltip("Multi-model database",)
      .with_label("SurrealDB",)
  }

  /// Local SVG asset — icon mark with brand gradient.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`SiSurrealdb`](icon::SiSurrealdb) with `--brand-surrealdb` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::SiSurrealdb,),)
      .colored("brand-surrealdb",)
  }

  /// Falls back to [`filled`] — no distinct outlined style exists.
  pub fn outlined() -> Icon { filled() }
}
