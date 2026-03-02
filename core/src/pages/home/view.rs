use {
  super::{
    about,
    contact,
    experience,
    hero,
    projects,
    tech,
  },
  crate::prelude::*,
};

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <hero::Hero />
    <div class="readable">
      <about::About />
      <tech::Stacks />
      <tech::Areas />
      <projects::Projects />
      <experience::Experience />
      <about::Philosophy />
      <contact::Contact />
    </div>
  }
}
