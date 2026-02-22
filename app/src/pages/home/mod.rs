mod about;
mod contact;
// mod dev;
mod experience;
mod hero;
mod projects;

use crate::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <hero::Hero />
    <div class="readable">
      <about::About />
      <projects::Projects />
      <experience::Experience />
      <about::Philosophy />
      <contact::Contact />
    </div>
  }
}
