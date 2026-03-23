use crate::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact">
      <h2>"📫 Let's Connect"</h2>
      <p class="contact__lead">
        "Looking for " <strong>"rust-centric projects"</strong> " across the full spectrum:"
      </p>
      <ul class="contact__list">
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
    </section>
  }
}
