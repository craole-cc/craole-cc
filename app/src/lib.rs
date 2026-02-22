mod base;
pub use base::*;

pub mod components;
pub mod constants;
pub mod layout;
pub mod pages;

pub mod prelude {
  pub use {
    crate::{
      base::*,
      components::*,
      constants::prelude::*,
      layout::{
        self,
        *,
      },
      pages::*,
      //
    },
    leptos::prelude::*,
    paste_complete::paste,
  };
}
