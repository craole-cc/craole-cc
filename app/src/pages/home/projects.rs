use crate::prelude::*;

// Project data is page-local — not reused elsewhere.
#[derive(Clone)]
struct Project {
  title: &'static str,
  description: &'static str,
  tags: Vec<&'static str>,
  status: &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
  let projects = vec![
    Project {
      title: "Full-Stack Portfolio",
      description: "Rust-powered portfolio site with Leptos + Axum, showcasing modern \
                    full-stack development with HTMX and Tailwind",
      tags: vec!["Rust", "Leptos", "Axum", "HTMX", "Tailwind"],
      status: "🚀 Active",
    },
    Project {
      title: "Data Pipeline System",
      description: "High-performance data pipeline using Rust and Delta Lake for efficient \
                    data ingestion and transformation",
      tags: vec!["Rust", "Delta Lake", "Apache Spark"],
      status: "🔨 Building",
    },
    Project {
      title: "Graph Analytics Platform",
      description: "Analytics platform using Neo4j to model complex relationships in \
                    business data",
      tags: vec!["Neo4j", "Cypher", "Rust"],
      status: "🔨 Building",
    },
    Project {
      title: "CLI Utilities Suite",
      description: "Cross-platform developer productivity tools and system utilities",
      tags: vec!["Rust", "CLI", "Cross-platform"],
      status: "💡 Planning",
    },
  ];

  view! {
    <section id="projects" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {}", NEUTRAL_TEXT_800)>"🚀 Projects"</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        {projects
          .into_iter()
          .map(|project| view! { <ProjectCard project=project /> })
          .collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
  view! {
    <div class=format!(
      "p-6 rounded-xl border shadow-sm transition-all duration-300 \
      hover:shadow-lg hover:-translate-y-1 \
      group backdrop-blur {} {} hover:{}",
      NEUTRAL_BG_SURFACE,
      NEUTRAL_BORDER_300,
      PRIMARY_BORDER_400,
    )>
      <div class="flex justify-between items-start mb-3">
        <h3 class=format!(
          "text-2xl font-semibold transition-colors {} group-hover:{}",
          NEUTRAL_TEXT_800,
          PRIMARY_TEXT_600,
        )>{project.title}</h3>
        <span class="ml-2 text-xs font-medium whitespace-nowrap">{project.status}</span>
      </div>

      <p class=format!("mb-4 leading-relaxed {}", NEUTRAL_TEXT_600)>{project.description}</p>

      <div class="flex flex-wrap gap-2">
        {project
          .tags
          .into_iter()
          .map(|tag| {
            view! {
              <span class=format!(
                "py-1 px-3 text-xs font-medium rounded-md border {} {} {}",
                PRIMARY_TEXT_600,
                PRIMARY_BG_100,
                PRIMARY_BORDER_300,
              )>{tag}</span>
            }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}
