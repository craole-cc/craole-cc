mod about;
mod contact;
mod experience;
mod footer;
mod header;
// mod hero;
mod projects;

use leptos::{
  mount::mount_to_body,
  prelude::*,
};
pub use {
  about::*,
  contact::*,
  experience::*,
  footer::*,
  header::*,
  // hero::*,
  projects::*,
};

pub fn mount() {
  mount_to_body(view);
}

#[component]
fn app() -> impl IntoView {
  view! {
    <ThemeProvider>
      <Header />
      // <Hero/>
      <About />
      <Projects />
      <Experience />
      <Contact />
      <Footer />
    </ThemeProvider>
  }
}

#[component]
pub fn theme_provider(children: Children) -> impl IntoView {
  view! {
    <div class="min-h-screen bg-white transition-colors dark:bg-slate-900">
      <div class="py-12 px-6 mx-auto max-w-7xl">{children()}</div>
    </div>
  }
}
