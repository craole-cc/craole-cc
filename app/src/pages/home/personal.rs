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
        "Before software, I spent most of my adult life making music. I started on keyboards and
        guitar as a teenager, then settled on bass. Church choir and band gave me an early place to
        perform, and I formed a singing group while I was still a teenager. Music stayed at the
        centre of my life for the next 15–20 years, including long-term work with Skygrass, formerly
        Blu Grass in the Sky, No-maddz, Stone Dub, Protoje & The Indiggnation, and BLACK as COLE."
      </p>
      <p>
        "With BLACK as COLE, I released the single Musical Romance. My creative interests have always
        extended beyond music into the arts generally: performing, literary work, design, photography,
        and anything that gives an idea a form."
      </p>
      <p>
        "I studied Music Performance (Jazz) at the Edna Manley College of the Visual and Performing
        Arts while working full time in Tier 1 networking support. I eventually made a deliberate
        decision to pause music professionally and move into business process outsourcing, learning
        and development, BI, remote teaching, and software."
      </p>
      <p>
        "Since July 2021, I have delivered more than 5,000 remote English-learning sessions to
        approximately 1,800 students from more than 30 nationalities. The thread through all of this
        work is the same: learn a system deeply, communicate clearly, and make something useful."
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
