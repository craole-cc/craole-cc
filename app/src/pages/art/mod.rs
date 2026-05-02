mod card;
mod detail;
mod filter;
mod header;
mod mosaic;
mod related;
mod view;

pub use {card::*, detail::*, filter::*, header::*, mosaic::*, related::*, view::*};

pub mod prelude {
  pub use super::{Art as ArtPage, Detail as ArtDetail};
}
