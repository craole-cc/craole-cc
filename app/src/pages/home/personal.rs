use crate::prelude::*;

#[component]
pub fn About() -> impl IntoView {
  view! {
    <section id="about" class="about">
      <h2>"About Me"</h2>
      <p>
        "Music is my first love. It shaped most of my adult life, long before software did. I played
        keyboards, guitar, and bass with the church choir and band as a teenager, but bass is the one
        I fell for and chose as my primary instrument. By my late teens I'd formed my own singing group."
      </p>
      <p>
        "For fifteen to twenty years, music was the center of my life. I played and worked with
        Skygrass, formerly Blu Grass in the Sky, No-maddz, Stone Dub, Protoje & The Indiggnation, and
        BLACK as COLE, a reggae/neo-soul/jazz fusion project I founded. As founder, I worked across
        management, production, web development, videography, direction, and photography."
      </p>
      <p>
        "Technology was always there too. My first job was teaching Microsoft Office to working
        professionals. From there I moved into computer repair, IT studies, WINDALCO's Kirkvine
        powerhouse lab, and Music Performance (Jazz) at the Edna Manley College while holding down
        full-time Tier 1 networking support."
      </p>
      <p>
        "Eventually I moved from professional music into BPO, training and development, BI, TEFL, and
        now software, data, and AI engineering. None of it feels like dissonance. It's the same motif:
        learn a system deeply, communicate clearly, and build something useful."
      </p>
      <p>
        "Since July 2021, I have delivered more than 5,000 remote English-learning sessions to
        approximately 1,800 students from more than 30 nationalities. Code is another instrument for
        turning an idea into something real that adds value to someone's life."
      </p>
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
