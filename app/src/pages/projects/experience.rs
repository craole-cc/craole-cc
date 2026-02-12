use leptos::prelude::*;

#[derive(Clone)]
struct Experience {
  title: &'static str,
  period: &'static str,
  points: Vec<&'static str>,
}

#[component]
pub fn Experience() -> impl IntoView {
  let experiences = vec![
    Experience {
      title: "Data/Backend Developer (Building Portfolio)",
      period: "2024 - Present",
      points: vec![
        "Building modern data pipelines with Rust and Delta Lake",
        "Exploring graph databases and advanced SQL patterns",
        "Designing backend systems for data analytics",
      ],
    },
    Experience {
      title: "Learning & Development Specialist",
      period: "8+ years",
      points: vec![
        "Built training programs and analytics dashboards",
        "Specialized in Power BI and Tableau for business intelligence",
        "Managed data-driven decision making processes",
      ],
    },
    Experience {
      title: "TEFL Tutor",
      period: "3 years",
      points: vec![
        "Teaching professionals and students English communication",
        "Developing custom learning materials and curricula",
      ],
    },
  ];

  view! {
    <section id="experience" class="mb-20">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">"Experience"</h2>
      <div class="space-y-8">
        {experiences.into_iter().map(|exp| view! { <Card experience=exp /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

// Private component - only used in this section
#[component]
fn Card(experience: Experience) -> impl IntoView {
  view! {
    <div class="py-2 pl-6 border-l-4 border-blue-500">
      <h3 class="mb-2 text-2xl font-semibold text-slate-800 dark:text-slate-100">
        {experience.title}
      </h3>
      <div class="mb-4 text-sm font-medium text-blue-600 dark:text-blue-400">
        {experience.period}
      </div>
      <ul class="space-y-2">
        {experience
          .points
          .into_iter()
          .map(|point| {
            view! {
              <li class="flex items-start text-slate-700 dark:text-slate-300">
                <span class="mr-3 text-blue-500 dark:text-blue-400">"▹"</span>
                <span>{point}</span>
              </li>
            }
          })
          .collect::<Vec<_>>()}
      </ul>
    </div>
  }
}
