mod about;
mod contact;
mod experience;
mod hero;
mod projects;
mod tech;
mod view;

pub use {
  about::*,
  contact::*,
  experience::*,
  hero::*,
  projects::*,
  tech::*,
  view::*,
};

pub mod prelude {
  pub use super::Home as HomePage;
}
