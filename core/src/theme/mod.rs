mod components;
mod config;
mod context;
mod scripts;
mod utilities;

pub use {
  components::*,
  config::*,
  context::*,
  scripts::*,
  utilities::*,
};

pub mod prelude {
  pub use super::{
    Context as ThemeContext,
    Theme,
    components::prelude::*,
    scripts::INIT_SCRIPT as THEME_INIT,
    utilities::*,
  };
}
