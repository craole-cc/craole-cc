mod base;
mod components;
mod pages;
pub use base::*;
pub mod prelude {
  pub use crate::{base::*, components::*, pages::*};
  pub use leptos::prelude::*;
}
