mod about;
mod contact;
mod dev;
mod experience;
mod footer;
mod hero;
mod projects;

use {
  crate::Layout,
  about::*,
  contact::*,
  dev::*,
  experience::*,
  footer::*,
  hero::*,
  leptos::prelude::*,
  projects::*,
};

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
