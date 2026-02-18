use crate::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
  view! {
    <section class="text-center">

      // ~@ Name with decorative gradient
      <div class="mb-8">
        <h1 class="mb-6 text-6xl font-bold leading-tight text-transparent bg-clip-text from-teal-600 via-purple-600 to-teal-600 md:text-7xl dark:from-teal-400 dark:via-purple-400 dark:to-teal-400 bg-linear-to-r">
          <span>{AUTHOR_FIRSTNAME}</span>
          <span class="text-7xl md:text-8xl">" '"{AUTHOR_ALIAS}"' "</span>
          <span>{AUTHOR_SURNAME}</span>
        </h1>
      </div>

      // ~@ Role badges — each uses its own intentional brand color, not the site palette
      <div class="flex flex-wrap gap-3 justify-center mb-8">
        <span class="py-2 px-4 text-sm font-semibold text-orange-700 bg-orange-100 rounded-full border border-orange-300 dark:text-orange-300 dark:border-orange-700 dark:bg-orange-900/30">
          "🦀 Rust-Powered"
        </span>
        <span class=format!(
          "py-2 px-4 text-sm font-semibold rounded-full border {PRIMARY_TEXT_700} {PRIMARY_BG_100}",
        )>"📊 Data Engineer"</span>
        <span class="py-2 px-4 text-sm font-semibold text-purple-700 bg-purple-100 rounded-full border border-purple-300 dark:text-purple-300 dark:border-purple-700 dark:bg-purple-900/30">
          "🌐 Full Stack"
        </span>
        <span class="py-2 px-4 text-sm font-semibold text-green-700 bg-green-100 rounded-full border border-green-300 dark:text-green-300 dark:border-green-700 dark:bg-green-900/30">
          "⚙️ Systems"
        </span>
      </div>

      // ~@ Tagline
      <p class=format!(
        "px-4 mx-auto mb-6 max-w-4xl text-xl leading-relaxed md:text-2xl {NEUTRAL_TEXT_700}",
      )>
        "I build "
        <strong>"full-stack applications, data infrastructure, and systems utilities"</strong>
        ", usually with " <strong>"Rust"</strong> " at the core."
      </p>

      // ~@ Quote
      <p class=format!(
        "px-4 mx-auto max-w-3xl text-lg italic {NEUTRAL_TEXT_600}",
      )>"\"Code is another instrument of expression through structure\" 🎵⚙️"</p>

    </section>
  }
}
