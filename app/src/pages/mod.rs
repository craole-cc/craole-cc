pub mod admin;
pub mod art;
pub mod dev;
pub mod home;
pub mod log;

/// Metadata for a top-level site page.
#[derive(Debug, Clone, Copy, PartialEq, Eq,)]
pub struct Page {
  pub label :       &'static str,
  pub description : &'static str,
  pub path :        &'static str,
}

pub const PAGES : [Page; 4] = [
  Page {
    label :       "Home",
    description : "Building systems, capturing moments, expressing ideas",
    path :        "/",
  },
  Page {
    label :       "Dev",
    description : "Projects, systems & open source",
    path :        "/dev",
  },
  Page {
    label :       "Art",
    description : "Photography, music & video",
    path :        "/art",
  },
  Page {
    label :       "Log",
    description : "CV, blog & reflections",
    path :        "/log",
  },
];

pub mod prelude {
  pub use super::{
    PAGES,
    admin::prelude::*,
    art::prelude::*,
    dev::prelude::*,
    home::prelude::*,
    log::prelude::*,
  };
}
