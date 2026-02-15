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

mod prelude {
  pub use {crate::Layout, leptos::prelude::*};
}
use prelude::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <Layout>
      <Hero />
      <About />
      <Areas />
      <Stacks />
      <Projects />
      <Experience />
      <Philosophy />
      <Contact />
      <Footer />
    </Layout>
  }
}
