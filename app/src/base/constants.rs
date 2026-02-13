/// Core facets that define the portfolio
#[derive(Debug, Clone, Copy)]
pub struct Facet {
  pub label: &'static str,
  pub description: &'static str,
  pub slug: &'static str, // For routing: /code, /data, etc.
}

pub const FACETS: [Facet; 4] = [
  Facet {
    label: "CODE",
    description: "Building systems & applications",
    slug: "code",
  },
  Facet {
    label: "DATA",
    description: "Engineering pipelines & insights",
    slug: "data",
  },
  Facet {
    label: "TEFL",
    description: "Teaching English as a foreign language",
    slug: "tefl",
  },
  Facet {
    label: "LIFE",
    description: "Photos, music, thoughts & moments",
    slug: "life",
  },
];

/// Site identity
pub const AUTHOR_NAME: &str = "Craig Cole";
pub const AUTHOR_NICKNAME: &str = "Craole";
pub const COPYRIGHT_YEAR: &str = "2026";
