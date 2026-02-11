use leptos::prelude::*;

#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
}

#[component]
pub fn card() -> impl IntoView {
    let projects = vec![
        Project {
            title: "Data Pipeline System",
            description: "Built a scalable data pipeline using Rust and Delta Lake for efficient data ingestion and transformation",
            tags: vec!["Rust", "Delta Lake", "SQL"],
        },
        Project {
            title: "Graph Database Analytics",
            description: "Developed analytics platform using Neo4j to model complex relationships in business data",
            tags: vec!["Neo4j", "Cypher", "Data Modeling"],
        },
        Project {
            title: "BI Dashboard Suite",
            description: "Created comprehensive Power BI dashboards for business intelligence and reporting",
            tags: vec!["Power BI", "Tableau", "SQL"],
        },
    ];

    view! {
      <section id="projects" class="mb-20">
        <h2 class="mb-8 text-4xl font-bold text-blue-400">"Projects"</h2>
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
          {projects
            .into_iter()
            .map(|project| view! { <ProjectCard project=project /> })
            .collect::<Vec<_>>()}
        </div>
      </section>
    }
}

#[component]
fn card(project: Project) -> impl IntoView {
    view! {
      <div class="p-6 rounded-xl border transition-all duration-300 hover:-translate-y-1 group bg-slate-800/50 backdrop-blur border-slate-700/50 hover:border-blue-500/50">
        <h3 class="mb-3 text-2xl font-semibold transition-colors group-hover:text-blue-400 text-slate-100">
          {project.title}
        </h3>
        <p class="mb-6 leading-relaxed text-slate-400">{project.description}</p>
        <div class="flex flex-wrap gap-2">
          {project
            .tags
            .into_iter()
            .map(|tag| {
              view! {
                <span class="py-1 px-3 text-sm font-medium text-blue-400 rounded-md border bg-slate-900/50 border-slate-700">
                  {tag}
                </span>
              }
            })
            .collect::<Vec<_>>()}
        </div>
      </div>
    }
}
