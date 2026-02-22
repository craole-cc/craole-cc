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
    <layout::Page>
      <hero::Hero />
      <layout::Content>
        <about::About />
        <projects::Projects />
        <experience::Experience />
        <about::Philosophy />
        <contact::Contact />
      </layout::Content>
    </layout::Page>
  }
}
