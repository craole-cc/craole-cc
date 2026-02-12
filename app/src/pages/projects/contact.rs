use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact" class="mb-16">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">
        "📫 Let's Connect"
      </h2>
      <p class="mx-auto mb-6 max-w-2xl text-lg text-center text-slate-700 dark:text-slate-300">
        "Looking for " <strong>"Rust-centric projects"</strong> " across the full spectrum:"
      </p>
      <ul class="mx-auto mb-10 space-y-2 max-w-2xl text-center text-slate-600 dark:text-slate-400">
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
      <div class="flex flex-wrap gap-4 justify-center">
        <a
          href="mailto:craig.craole.cole@gmail.com"
          class="py-4 px-8 font-semibold text-white bg-blue-600 rounded-lg border-2 border-blue-600 transition-all duration-300 dark:bg-blue-500 dark:border-blue-500 hover:bg-blue-700 hover:border-blue-700 dark:hover:bg-blue-600"
          target="_blank"
        >
          "📧 Email"
        </a>
        <a
          href="https://github.com/craole-cc"
          class="py-4 px-8 font-semibold text-blue-600 bg-white rounded-lg border-2 border-blue-500 transition-all duration-300 dark:text-blue-400 dark:border-blue-500 hover:text-white hover:bg-blue-600 hover:border-blue-600 dark:bg-slate-800 dark:hover:bg-blue-500"
          target="_blank"
        >
          "GitHub"
        </a>
        <a
          href="https://twitter.com/craole"
          class="py-4 px-8 font-semibold text-blue-600 bg-white rounded-lg border-2 border-blue-500 transition-all duration-300 dark:text-blue-400 dark:border-blue-500 hover:text-white hover:bg-blue-600 hover:border-blue-600 dark:bg-slate-800 dark:hover:bg-blue-500"
          target="_blank"
        >
          "𝕏 Twitter"
        </a>
      </div>
    </section>
  }
}
