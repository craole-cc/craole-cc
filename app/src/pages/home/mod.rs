pub mod about;
use about::*;
pub mod contact;
use contact::*;
pub mod dev;
use dev::*;
pub mod experience;
use experience::*;
pub mod footer;
use footer::*;
pub mod hero;
use hero::*;
pub mod projects;
use projects::*;

use crate::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <layout::Basic>
      <Hero />
      <Divider />
      <About />
      <Areas />
      <Stacks />
      <Projects />
      <Experience />
      <Philosophy />
      <Contact />
      <Footer />
    </layout::Basic>
  }
}
