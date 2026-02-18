mod base;
pub mod layout;
pub use base::*;

pub mod components;
pub mod constants;
pub mod pages;

pub mod prelude {
  pub use {
    crate::{
      base::*,
      components::*,
      constants::prelude::*,
      layout,
      pages::*,
      //
    },
    leptos::prelude::*,
    paste_complete::paste,
    singlestage::*,
  };
}
