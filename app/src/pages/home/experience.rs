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
      title :  "English as a Second Language Specialist",
      period : "Freelance and remote · July 2021 – Present",
      points : vec![
        "Delivered more than 5,000 remote learning sessions to approximately 1,800 students from \
         more than 30 nationalities",
        "Prepared learners for IELTS, TOEFL, interviews, presentations, and business meetings",
        "Built structured, goal-oriented learning environments through coaching and feedback",
      ],
    },
    Experience {
      title :  "Business Intelligence Specialist & Training / Quality Coordinator",
      period : "NKCS · June 2019 – February 2021",
      points : vec![
        "Designed, implemented, and maintained BI tools and CRM systems across departments",
        "Analyzed business requirements, processes, performance indicators, and improvement areas",
        "Oversaw the rollout of a Training and Quality department during startup operations",
        "Designed and delivered leadership, skills, BI-tool, and CRM-system training",
      ],
    },
    Experience {
      title :  "Learning & Development Officer",
      period : "HGS · October 2016 – June 2019",
      points : vec![
        "Supported design, testing, implementation, and local support for a global \
         learning-management system",
        "Designed development programs aligned with organizational goals and identified training \
         needs",
        "Trained leaders and brand advocates in leadership, facilitation, Microsoft Office, \
         customer experience, and sales strategy",
        "Supported technical and creative design of forms, dashboards, and print media",
      ],
    },
    Experience {
      title :  "Process Trainer / Technical Support Brand Advocate",
      period : "HGS · September 2015 – January 2017",
      points : vec![
        "Coached production teams through targeted training interventions and quality improvement \
         projects",
        "Participated in Kaizen events to reduce technical-support errors",
        "Supported Internet, VoIP, and networking issues for Global Capacity and Megapath \
         customers",
        "Maintained a 98% matriculation-to-production rate across five technical-support training \
         cohorts",
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
