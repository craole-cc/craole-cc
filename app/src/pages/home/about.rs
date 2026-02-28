use crate::prelude::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <section id="about" class="about">
      <h2>"👤 About Me"</h2>
      <p>
        "From frontend interfaces to backend pipelines to command-line tools, I'm
          exploring the entire spectrum of what's possible with modern systems
          programming."
      </p>
      <p>
        "With roots in " <strong>"music production"</strong>
        " (bassist, singer, producer) and 8+ years in " <strong>"Learning & Development"</strong>
        " (including TEFL tutoring for professionals), I've always been driven by "
        <strong>"expression"</strong> ". The alias " <strong>{AUTHOR_ALIAS}</strong>
        " embodies this — a fusion of my Caribbean heritage and creative identity,
        reminding me that " <em>"I have something to say"</em> "."
      </p>
      <p>
        "Music remains part of my voice, but " <strong>"code has expanded my range"</strong>
        ". Every system I build, every pipeline I design, every utility I craft —
        it's all " <strong>"expression through structure"</strong>
        ", blending the precision and creativity I honed as a musician with the
        power of modern systems programming."
      </p>
      <blockquote>
        "Code is another instrument of expression through structure 🎵⚙️"
      </blockquote>
    </section>
  }
}

#[component]
pub fn Philosophy() -> impl IntoView {
  view! {
    <section id="philosophy" class="philosophy">
      <h2>"🎯 Philosophy"</h2>
      <div class="philosophy__card">
        <p>
          "Code is expression. Just like music, it requires "
          <strong>"precision, creativity, and purpose."</strong>
          " Whether I'm building a web app, designing a data pipeline, or crafting a
          CLI tool, it's about solving problems in ways that feel "
          <strong>"structured yet innovative"</strong> "."
        </p>
        <p>
          "From my musical background to teaching to building systems — it's all
          connected through the desire to " <strong>"create and communicate"</strong> "."
        </p>
      </div>
    </section>
  }
}
