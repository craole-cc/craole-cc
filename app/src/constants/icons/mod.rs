mod components;
pub use components::*;
mod registry;
pub use registry::*;
mod core;
pub use core::*;
mod source;
pub use source::*;

pub mod prelude {
  pub use super::{Icon, Icons, Render as IconRender, Source as IconSource};
}

pub(crate) use {
  icondata::{self as icon, Icon as IconData},
  leptos_icons::Icon as LeptosIcon,
};
