#![recursion_limit = "256"]
mod base;
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
      pages::*,
    },
    leptos::prelude::*,
    paste_complete::paste,
  };
  // #[cfg(feature = "hydrate")]
  pub use {
    wasm_bindgen::{
      JsCast,
      closure::Closure,
    },
    wasm_bindgen_futures::spawn_local,
    web_sys::{
      HtmlCanvasElement,
      HtmlImageElement,
      js_sys,
      window,
    },
  };
}
