mod card;
mod detail;
mod filter;
mod grid;
mod header;
mod view;

pub use {card::*, detail::*, filter::*, grid::*, header::*, view::*};

pub mod _prelude {
  pub use {super::Card, crate::prelude::*};
}

pub mod prelude {
  pub use super::{Detail as DevDetail, Dev as DevPage};
}
