mod core;
pub mod registry;
mod utilities;
mod variant;

pub use {
  core::*,
  registry::*,
  utilities::*,
  variant::*,
};

pub mod prelude {
  pub use super::{
    Icon,
    Render as IconRender,
    Source as IconSource,
    Variant as IconVariant,
    registry as icons,
  };
}
