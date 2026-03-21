use crate::prelude::*;

struct Discipline {
  number :      &'static str,
  title :       &'static str,
  description : &'static str,
  path :        &'static str,
  image :       &'static str,
}

const DISCIPLINES: &[Discipline] = &[
  Discipline {
    number:      "01",
    title:       "Dev",
    description: "Architectural code built with precision and durability for complex problem-solving.",
    path:        "/dev",
    image:       "https://images.unsplash.com/photo-1587620962725-abab7fe55159?auto=format&fit=crop&w=1200&q=80",
  },
  Discipline {
    number:      "02",
    title:       "Art",
    description: "Visual narratives that transcend the screen. Exploring the intersection of digital medium and human emotion.",
    path:        "/art",
    image:       "https://images.unsplash.com/photo-1518998053901-5348d3961a04?auto=format&fit=crop&w=1200&q=80",
  },
  Discipline {
    number:      "03",
    title:       "Log",
    description: "Documenting the journey. A technical repository of experiments, thoughts, and architectural insights.",
    path:        "/log",
    image:       "https://images.unsplash.com/photo-1471107340929-a87cd0f5b5f3?auto=format&fit=crop&w=1200&q=80",
  },
];

#[component]
#[must_use]
pub fn Disciplines() -> impl IntoView {
  view! {
    <section class="disciplines">
      {DISCIPLINES
        .iter()
        .map(|d| {
          view! {
            <a href=d.path class="discipline">
              <div class="discipline__bg" style=format!("background-image:url('{}')", d.image) />
              <div class="discipline__inner">
                <span class="discipline__number">{d.number}</span>
                <div class="discipline__body">
                  <h2 class="discipline__title">{d.title}</h2>
                  <p class="discipline__description">{d.description}</p>
                </div>
              </div>
            </a>
          }
        })
        .collect_view()}
    </section>
  }
}
