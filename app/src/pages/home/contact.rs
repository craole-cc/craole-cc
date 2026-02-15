use crate::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">
        "📫 Let's Connect"
      </h2>
      <p class="mx-auto mb-6 max-w-2xl text-lg text-center text-slate-700 dark:text-slate-300">
        "Looking for " <strong>"rust-centric projects"</strong> " across the full spectrum:"
      </p>
      <ul class="mx-auto mb-6 space-y-2 max-w-2xl text-center text-slate-600 dark:text-slate-400">
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
    </section>
  }
}
