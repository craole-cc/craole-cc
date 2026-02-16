mod config;
pub use config::*;
mod registry;
pub use registry::*;
mod utilities;
pub use utilities::*;

pub mod prelude {
  pub use super::{
    Icon,
    Icons,
    Render as IconRender,
  };
}

use {
  icondata as ico,
  leptos_icons::Icon as LeptosIcon,
};
