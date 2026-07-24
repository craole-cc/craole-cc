use crate::prelude::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <section id="about" class="about">
      <h2>"👤 About Me"</h2>
      <p>
        "I'm Craig " <strong>{AUTHOR_ALIAS}</strong> " Cole, a BI and data professional
        transitioning into Rust-first software, data, and AI engineering. I build production-minded
        applications, data workflows, and systems utilities with a focus on reproducibility,
        testing, and clear documentation."
      </p>
      <p>
        "Music was not a side interest in my life—it was my first professional identity. I started
        as a teenager on keyboards and guitar before choosing bass. I sang and played in church,
        formed a singing group as a teenager, and later spent approximately 15–20 years working as
        a professional musician."
      </p>
      <p>
        "I studied Music Performance (Jazz) at Edna Manley College while working full time in Tier 1
        networking support. I later paused music professionally and moved fully into BPO, learning
        and development, BI, and software. Music, design, and technology have been connected
        throughout my life."
      </p>
      <p>
        "Since July 2021, I have delivered more than 5,000 remote English-learning sessions to
        approximately 1,800 students from more than 30 nationalities. I bring the same habits to
        software that I bring to teaching and operations: understand the user's goal, make the
        system explainable, test the important paths, and finish the work."
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
          "Good engineering connects people, data, and dependable systems. It requires "
          <strong>"precision, creativity, and purpose."</strong>
          " Whether I'm building a web application, designing a data workflow, or maintaining
          infrastructure, I aim for solutions that are understandable, testable, and useful."
        </p>
        <p>
          "From music and teaching to BI and software, the common thread is the desire to "
          <strong>"create and communicate"</strong> "."
        </p>
      </div>
    </section>
  }
}

#[component]
#[must_use]
pub fn Vision() -> impl IntoView {
  view! {
    <section class="vision">
      <div class="vision__inner">
        <div class="vision__label">
          <span>"The Vision"</span>
        </div>
        <div class="vision__statement">
          <p>
            "Raised on rhythm, shaped by design, and grounded in technical work, I carried the habits of
            a professional musician—listening closely, practicing deliberately, and collaborating
            under pressure—into teaching, BI, operations, and now reliable software and data systems."
          </p>
        </div>
      </div>
    </section>
  }
}
