use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <section id="about" class="mb-20">
      <h2 class="mb-8 text-4xl font-bold text-slate-900 dark:text-blue-400">
        "👤 About Me"
      </h2>
      <div class="space-y-6 text-lg">
        <p class="leading-relaxed text-slate-700 dark:text-slate-300">
          "From frontend interfaces to backend pipelines to command-line tools, I'm exploring the entire spectrum of what's possible with modern systems programming."
        </p>

        <p class="leading-relaxed text-slate-700 dark:text-slate-300">
          "With roots in " <strong>"music production"</strong> " (bassist, singer, producer) and 8+ years in "
          <strong>"Learning & Development"</strong> " (including TEFL tutoring for professionals), I've always been driven by "
          <strong>"expression"</strong> ". The alias " <strong>"\"Craole\""</strong> " embodies this — a fusion of my Caribbean heritage and creative identity, reminding me that "
          <em>"I have something to say"</em> "."
        </p>

        <p class="leading-relaxed text-slate-700 dark:text-slate-300">
          "Music remains part of my voice, but " <strong>"code has expanded my range"</strong>
          ". Every system I build, every pipeline I design, every utility I craft — it's all "
          <strong>"expression through structure"</strong>
          ", blending the precision and creativity I honed as a musician with the power of modern systems programming."
        </p>

        <blockquote class="pl-4 italic border-l-4 border-blue-500 dark:border-blue-400 text-slate-600 dark:text-slate-400">
          "Code is another instrument of expression through structure 🎵⚙️"
        </blockquote>
      </div>
    </section>
  }
}

#[component]
pub fn Philosophy() -> impl IntoView {
  view! {
    <section id="philosophy" class="mb-20">
      <h2 class="mb-8 text-4xl font-bold text-slate-900 dark:text-blue-400">
        "🎯 Philosophy"
      </h2>
      <div class="p-8 border-l-4 border-blue-500 rounded-r-xl bg-linear-to-r from-blue-50 to-transparent dark:from-blue-900/20 dark:border-blue-400">
        <p class="mb-4 text-xl leading-relaxed text-slate-700 dark:text-slate-300">
          "Code is expression. Just like music, it requires "
          <strong>"precision, creativity, and purpose"</strong>
          ". Whether I'm building a web app, designing a data pipeline, or crafting a CLI tool, it's about solving problems in ways that feel "
          <strong>"structured yet innovative"</strong> "."
        </p>
        <p class="text-lg leading-relaxed text-slate-600 dark:text-slate-400">
          "From my musical background to teaching to building systems—it's all connected through the desire to "
          <strong>"create and communicate"</strong> "."
        </p>
      </div>
    </section>
  }
}
