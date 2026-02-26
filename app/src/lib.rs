#![recursion_limit = "256"]
mod base;
pub use base::*;

pub mod components;
pub mod constants;
pub mod pages;

pub mod prelude {
  pub use crate::{
    base::*,
    components::*,
    constants::prelude::*,
    pages::*,
  };
}

pub mod _prelude {
  pub use {
    crate::prelude::*,
    futures::channel::oneshot,
    icondata::{
      self as icon,
      Icon as IconData,
    },
    leptos::prelude::*,
    leptos_icons::Icon as LeptosIcon,
    paste_complete::paste,
    std::cell::Cell,
    wasm_bindgen::{
      JsCast,
      closure::Closure,
    },
    wasm_bindgen_futures::spawn_local,
    web_sys::{
      CanvasRenderingContext2d,
      HtmlCanvasElement,
      HtmlElement,
      HtmlImageElement,
      MediaQueryList,
      MouseEvent,
      js_sys,
      window,
    },
  };
}
