use crate::prelude::*;

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
      description: "Rust-powered portfolio site with Leptos + Axum, showcasing modern full-stack development with HTMX and Tailwind",
      tags: vec!["Rust", "Leptos", "Axum", "HTMX", "Tailwind"],
      status: "🚀 Active",
    },
    Project {
      title: "Data Pipeline System",
      description: "High-performance data pipeline using Rust and Delta Lake for efficient data ingestion and transformation",
      tags: vec!["Rust", "Delta Lake", "Apache Spark"],
      status: "🔨 Building",
    },
    Project {
      title: "Graph Analytics Platform",
      description: "Analytics platform using Neo4j to model complex relationships in business data",
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
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">"🚀 Projects"</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        {projects.into_iter().map(|project| view! { <Card project=project /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn Card(project: Project) -> impl IntoView {
  view! {
    <div class="p-6 rounded-xl border shadow-sm transition-all duration-300 hover:border-blue-400 hover:shadow-lg hover:-translate-y-1 group bg-white/80 backdrop-blur border-slate-200 dark:bg-slate-800/50 dark:border-slate-700/50 dark:hover:border-blue-500/50">
      <div class="flex justify-between items-start mb-3">
        <h3 class="text-2xl font-semibold transition-colors group-hover:text-blue-600 text-slate-800 dark:text-slate-100 dark:group-hover:text-blue-400">
          {project.title}
        </h3>
        <span class="ml-2 text-xs font-medium whitespace-nowrap">{project.status}</span>
      </div>
      <p class="mb-4 leading-relaxed text-slate-600 dark:text-slate-400">{project.description}</p>
      <div class="flex flex-wrap gap-2">
        {project
          .tags
          .into_iter()
          .map(|tag| {
            view! {
              <span class="py-1 px-3 text-xs font-medium text-blue-600 bg-blue-50 rounded-md border border-blue-200 dark:text-blue-400 dark:bg-slate-900/50 dark:border-slate-700">
                {tag}
              </span>
            }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}
