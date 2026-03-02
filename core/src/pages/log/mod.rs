mod hero;
mod post;
mod utils;
mod view;

pub use {
  hero::*,
  post::*,
  view::*,
};
pub mod _prelude {
  pub use {
    super::utils::render_markdown,
    crate::prelude::*,
  };
}

pub mod prelude {
  pub use super::{
    Log as LogPage,
    Post as LogPost,
  };
}
