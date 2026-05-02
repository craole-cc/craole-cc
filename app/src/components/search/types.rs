use super::_prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
  pub title: String,
  pub subtitle: String,
  pub kind: Kind,
  pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
  Project,
  Art,
  Log,
}
