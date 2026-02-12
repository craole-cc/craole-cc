use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
  view! {
    <section class="py-20 mb-16 text-center border-b border-slate-200 dark:border-slate-700/50">
      // Avatar - removed, cleaner look

      <div class="mb-8">
        <h1 class="mb-6 text-6xl font-bold leading-tight text-transparent md:text-7xl bg-clip-text bg-linear-to-r from-blue-600 via-purple-600 to-blue-600 dark:from-blue-400 dark:via-purple-400 dark:to-blue-400">
          "Craig 'Craole' Cole"
        </h1>
      </div>

      <div class="flex flex-wrap justify-center gap-3 mb-8">
        <span class="px-4 py-2 text-sm font-semibold text-orange-700 bg-orange-100 border border-orange-300 rounded-full dark:text-orange-300 dark:bg-orange-900/30 dark:border-orange-700">
          "🦀 Rust-Powered"
        </span>
        <span class="px-4 py-2 text-sm font-semibold text-blue-700 bg-blue-100 border border-blue-300 rounded-full dark:text-blue-300 dark:bg-blue-900/30 dark:border-blue-700">
          "📊 Data Engineer"
        </span>
        <span class="px-4 py-2 text-sm font-semibold text-purple-700 bg-purple-100 border border-purple-300 rounded-full dark:text-purple-300 dark:bg-purple-900/30 dark:border-purple-700">
          "🌐 Full Stack"
        </span>
        <span class="px-4 py-2 text-sm font-semibold text-green-700 bg-green-100 border border-green-300 rounded-full dark:text-green-300 dark:bg-green-900/30 dark:border-green-700">
          "⚙️ Systems"
        </span>
      </div>

      <p class="max-w-4xl px-4 mx-auto mb-6 text-xl leading-relaxed md:text-2xl text-slate-700 dark:text-slate-300">
        "I build " <strong>"full-stack applications, data infrastructure, and systems utilities"</strong>
        ", usually with " <strong>"Rust"</strong> " at the core."
      </p>

      <p class="max-w-3xl px-4 mx-auto text-lg italic text-slate-600 dark:text-slate-400">
        "\"Code is another instrument of expression through structure\" 🎵⚙️"
      </p>
    </section>
  }
}
