mod core;
mod registry;
mod utilities;

pub mod prelude {
  pub use super::{
    Icon,
    Icons,
    Render as IconRender,
    Source as IconSource,
  };
}

use crate::prelude::*;
pub use {
  core::*,
  icondata::{
    self as icon,
    Icon as IconData,
  },
  leptos_icons::Icon as LeptosIcon,
  registry::*,
  utilities::*,
};
