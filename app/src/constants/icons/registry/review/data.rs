use crate::constants::icons::Icon;

const DARK_INVERT : &str = "dark:invert dark:hue-rotate-180";

pub fn deltalake() -> Icon {
  Icon::new_local("icons/logos/deltalake.svg",)
    .with_class(DARK_INVERT,)
    .with_label("Delta Lake",)
}

pub fn surrealdb() -> Icon {
  Icon::new_local("icons/logos/surrealdb.png",).with_label("SurrealDB",)
}

pub fn neo4j() -> Icon { Icon::new_local("icons/logos/neo4j-flat.svg",).with_label("Neo4j",) }

pub fn postgresql() -> Icon {
  Icon::new_local("icons/logos/postgresql.svg",).with_label("PostgreSQL",)
}

pub fn sqlite() -> Icon {
  Icon::new_local("icons/logos/SQLite.svg",)
    .with_class(DARK_INVERT,)
    .with_label("SQLite",)
}
