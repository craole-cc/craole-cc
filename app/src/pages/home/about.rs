use crate::prelude::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <section id="about" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {NEUTRAL_TEXT_800}")>"👤 About Me"</h2>
      <div class="space-y-6 text-lg">
        <p class=format!(
          "leading-relaxed {NEUTRAL_TEXT_700}",
        )>
          "From frontend interfaces to backend pipelines to command-line tools, I'm exploring
          the entire spectrum of what's possible with modern systems programming."
        </p>

        <p class=format!(
          "leading-relaxed {NEUTRAL_TEXT_700}",
        )>
          "With roots in " <strong>"music production"</strong>
          " (bassist, singer, producer) and 8+ years in " <strong>"Learning & Development"</strong>
          " (including TEFL tutoring for professionals), I've always been driven by "
          <strong>"expression"</strong> ". The alias " <strong>"\"Craole\""</strong>
          " embodies this — a fusion of my Caribbean heritage and creative identity, reminding me that "
          <em>"I have something to say"</em> "."
        </p>

        <p class=format!(
          "leading-relaxed {NEUTRAL_TEXT_700}",
        )>
          "Music remains part of my voice, but " <strong>"code has expanded my range"</strong>
          ". Every system I build, every pipeline I design, every utility I craft — it's all "
          <strong>"expression through structure"</strong>
          ", blending the precision and creativity I honed as a musician with the power of
          modern systems programming."
        </p>

        <blockquote class=format!(
          "pl-4 italic border-l-4 {PRIMARY_BORDER_500} {NEUTRAL_TEXT_600}",
        )>"Code is another instrument of expression through structure 🎵⚙️"</blockquote>
      </div>
    </section>
  }
}

#[component]
pub fn Philosophy() -> impl IntoView {
  view! {
    <section id="philosophy" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {NEUTRAL_TEXT_800}")>"🎯 Philosophy"</h2>
      <div class=format!(
        "p-8 {PRIMARY_BG_100} rounded-r-xl border-l-4 {PRIMARY_BORDER_500} bg-linear-to-r to-transparent",
      )>
        <p class=format!(
          "mb-4 text-xl leading-relaxed {NEUTRAL_TEXT_700}",
        )>
          "Code is expression. Just like music, it requires "
          <strong>"precision, creativity, and purpose."</strong>
          " Whether I'm building a web app, designing a data pipeline, or crafting a CLI tool,
          it's about solving problems in ways that feel "
          <strong>"structured yet innovative"</strong> "."
        </p>
        <p class=format!(
          "text-lg leading-relaxed {NEUTRAL_TEXT_600}",
        )>
          "From my musical background to teaching to building systems — it's all connected
          through the desire to " <strong>"create and communicate"</strong> "."
        </p>
      </div>
    </section>
  }
}
