mod components;
pub use components::*;
mod registry;
pub use registry::*;
mod icon;
pub use icon::*;
mod source;
pub use source::*;
mod variant;
pub use variant::*;

pub mod prelude {
  pub use super::{
    Icon,
    Icons,
    Render as IconRender,
    Source as IconSource,
    Variant as IconVariant,
  };
}

pub(crate) use {
  icondata::{
    self as ico,
    Icon as IconData,
  },
  leptos_icons::Icon as LeptosIcon,
};
