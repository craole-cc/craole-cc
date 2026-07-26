mod contact;
mod disciplines;
mod experience;
mod hero;
mod personal;
mod projects;
mod tech;
mod view;

pub use {
  contact::*,
  disciplines::*,
  experience::*,
  hero::*,
  personal::*,
  projects::*,
  tech::*,
  view::*,
};

pub mod _prelude {
  pub use {
    super::*,
    crate::prelude::*,
  };
}

pub mod prelude {
  pub use super::Home as HomePage;
}
