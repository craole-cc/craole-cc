mod about;
mod contact;
mod disciplines;
mod experience;
mod hero;
mod projects;
mod tech;
mod view;
mod vision;

pub use {
  about::*,
  contact::*,
  // cta::*,
  disciplines::*,
  experience::*,
  hero::*,
  projects::*,
  tech::*,
  view::*,
  vision::*,
};

pub mod prelude {
  pub use super::Home as HomePage;
}
