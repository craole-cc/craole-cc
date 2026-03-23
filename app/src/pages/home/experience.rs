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
      period : "2024 – Present",
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
    <section id="experience" class="experience">
      <h2>"Experience"</h2>
      <div class="experience__list">
        {experiences.into_iter().map(|e| view! { <ExperienceCard experience=e /> }).collect_view()}
      </div>
    </section>
  }
}

#[component]
fn ExperienceCard(experience : Experience,) -> impl IntoView {
  view! {
    <article class="experience-card">
      <h3>{experience.title}</h3>
      <p>{experience.period}</p>
      <ul role="list">
        {experience.points.into_iter().map(|point| view! { <li>{point}</li> }).collect_view()}
      </ul>
    </article>
  }
}
