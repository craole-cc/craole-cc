#![allow(clippy::must_use_candidate)]
use super::_prelude::*;

#[component]
pub fn Home() -> impl IntoView {
  view! {
    <div id="home" class="home">
      <Hero />
      <main class="readable">
        <Vision />
        <Disciplines />
        <Areas />
      </main>
    </div>
  }
}
