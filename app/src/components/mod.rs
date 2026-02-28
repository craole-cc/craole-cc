mod dividers;
mod footer;
mod header;

pub use {
  dividers::*,
  footer::*,
  header::*,
};

pub mod prelude {
  pub use super::{
    dividers::*,
    footer::Footer,
    header::Header,
  };
}
