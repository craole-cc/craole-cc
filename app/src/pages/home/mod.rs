mod about;
mod contact;
mod dev;
mod experience;
mod footer;
mod hero;
mod projects;

use crate::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <layout::Basic>
      <hero::Hero />
      <Divider />
      <about::About />
      <dev::Areas />
      <dev::Stacks />
      <projects::Projects />
      <experience::Experience />
      <about::Philosophy />
      <contact::Contact />
      <footer::Footer />
    </layout::Basic>
  }
}
