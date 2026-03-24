mod field;
mod hooks;
mod result;
mod results;
mod server;
mod tags;
mod trigger;
mod types;
mod view;

pub mod _prelude {
  pub use {
    super::*,
    crate::prelude::*,
  };
}

pub use {
  field::*,
  hooks::*,
  result::*,
  results::*,
  server::*,
  tags::*,
  trigger::*,
  types::*,
  view::*,
};

pub mod prelude {
  pub use super::{
    Item as SearchResult,
    Kind as SearchKind,
    Search,
    Tags as ContextTags,
  };
}
