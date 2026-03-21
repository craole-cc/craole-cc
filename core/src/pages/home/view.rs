#![allow(clippy::must_use_candidate)]
use {
  super::{
    // projects,
    // tech,
    disciplines,
    // about,
    // contact,
    // experience,
    hero,
    vision,
  },
  crate::prelude::*,
};

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <hero::Hero />
    <div class="readable">
      <vision::Vision />
      <disciplines::Disciplines />
    // <cta::Cta />
    // <about::About />
    // <tech::Stacks />
    // <tech::Areas />
    // <projects::Projects />
    // <experience::Experience />
    // <about::Philosophy />
    // <contact::Contact />
    </div>
  }
}
