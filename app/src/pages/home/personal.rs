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
        "My professional background combines business intelligence, CRM and training-system work,
        learning and development, technical support, and remote language teaching. Since July 2021,
        I have delivered more than 5,000 remote English-learning sessions to approximately 1,800
        students from more than 30 nationalities."
      </p>
      <p>
        "I bring the same habits to software that I bring to teaching and operations: understand the
        user's goal, make the system explainable, test the important paths, and finish the work."
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
            "Raised on rhythm and shaped by teaching, BI, and operations, I now express the same
            care through reliable software and data systems."
          </p>
        </div>
      </div>
    </section>
  }
}
