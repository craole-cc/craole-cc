use crate::prelude::*;

#[derive(Clone,)]
struct Experience {
  title :  &'static str,
  period : &'static str,
  points : Vec<&'static str,>,
}

#[component]
pub fn Experience() -> impl IntoView {
  let experiences = vec![
    Experience {
      title :  "Data/Backend Developer (Building Portfolio)",
      period : "2024 - Present",
      points : vec![
        "Building modern data pipelines with Rust and Delta Lake",
        "Exploring graph databases and advanced SQL patterns",
        "Designing backend systems for data analytics",
      ],
    },
    Experience {
      title :  "Learning & Development Specialist",
      period : "8+ years",
      points : vec![
        "Built training programs and analytics dashboards",
        "Specialized in Power BI and Tableau for business intelligence",
        "Managed data-driven decision making processes",
      ],
    },
    Experience {
      title :  "TEFL Tutor",
      period : "3 years",
      points : vec![
        "Teaching professionals and students English communication",
        "Developing custom learning materials and curricula",
      ],
    },
  ];

  view! {
    <section id="experience" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {}", NEUTRAL_TEXT_800)>"Experience"</h2>
      <div class="space-y-8">
        {experiences
          .into_iter()
          .map(|exp| view! { <ExperienceCard experience=exp /> })
          .collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn ExperienceCard(experience : Experience,) -> impl IntoView {
  view! {
    <div class=format!("py-2 pl-6 border-l-4 {}", PRIMARY_BORDER_500)>
      <h3 class=format!("mb-2 text-2xl font-semibold {}", NEUTRAL_TEXT_800)>{experience.title}</h3>
      <div class=format!("mb-4 text-sm font-medium {}", PRIMARY_TEXT_600)>{experience.period}</div>
      <ul class="space-y-2">
        {experience
          .points
          .into_iter()
          .map(|point| {
            view! {
              <li class=format!("flex items-start {}", NEUTRAL_TEXT_700)>
                <span class=format!("mr-3 {}", PRIMARY_TEXT_600)>"▹"</span>
                <span>{point}</span>
              </li>
            }
          })
          .collect::<Vec<_>>()}
      </ul>
    </div>
  }
}
