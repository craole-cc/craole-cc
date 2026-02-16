use crate::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact">
      <h2 class=format!("mb-8 text-4xl font-bold {}", NEUTRAL_TEXT_800)>"📫 Let's Connect"</h2>
      <p class=format!(
        "mx-auto mb-6 max-w-2xl text-lg text-center {}",
        NEUTRAL_TEXT_700,
      )>"Looking for " <strong>"rust-centric projects"</strong> " across the full spectrum:"</p>
      <ul class=format!("mx-auto mb-6 space-y-2 max-w-2xl text-center {}", NEUTRAL_TEXT_600)>
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
    </section>
  }
}
