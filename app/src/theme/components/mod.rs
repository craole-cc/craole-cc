mod provider;
mod switcher;

use super::Context;
pub use {
  provider::*,
  switcher::*,
};

pub mod prelude {
  pub use super::{
    Provider as ThemeProvider,
    Switcher as ThemeSwitcher,
  };
}
