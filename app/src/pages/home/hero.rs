use crate::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
  view! {
    <section class="py-20 mb-16 text-center border-b border-slate-200 dark:border-slate-700/50">
      // Avatar

      <div class="mb-8">
        <h1 class="mb-6 text-6xl font-bold leading-tight text-transparent bg-clip-text from-blue-600 via-purple-600 to-blue-600 md:text-7xl dark:from-blue-400 dark:via-purple-400 dark:to-blue-400 bg-linear-to-r">
          "Craig 'Craole' Cole"
        </h1>
      </div>

      <div class="flex flex-wrap gap-3 justify-center mb-8">
        <span class="py-2 px-4 text-sm font-semibold text-orange-700 bg-orange-100 rounded-full border border-orange-300 dark:text-orange-300 dark:border-orange-700 dark:bg-orange-900/30">
          "🦀 Rust-Powered"
        </span>
        <span class="py-2 px-4 text-sm font-semibold text-blue-700 bg-blue-100 rounded-full border border-blue-300 dark:text-blue-300 dark:border-blue-700 dark:bg-blue-900/30">
          "📊 Data Engineer"
        </span>
        <span class="py-2 px-4 text-sm font-semibold text-purple-700 bg-purple-100 rounded-full border border-purple-300 dark:text-purple-300 dark:border-purple-700 dark:bg-purple-900/30">
          "🌐 Full Stack"
        </span>
        <span class="py-2 px-4 text-sm font-semibold text-green-700 bg-green-100 rounded-full border border-green-300 dark:text-green-300 dark:border-green-700 dark:bg-green-900/30">
          "⚙️ Systems"
        </span>
      </div>

      <p class="px-4 mx-auto mb-6 max-w-4xl text-xl leading-relaxed md:text-2xl text-slate-700 dark:text-slate-300">
        "I build "
        <strong>"full-stack applications, data infrastructure, and systems utilities"</strong>
        ", usually with " <strong>"Rust"</strong> " at the core."
      </p>

      <p class="px-4 mx-auto max-w-3xl text-lg italic text-slate-600 dark:text-slate-400">
        "\"Code is another instrument of expression through structure\" 🎵⚙️"
      </p>
    </section>
  }
}
