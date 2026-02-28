use crate::prelude::*;

#[derive(Clone,)]
struct Project {
  title :       &'static str,
  description : &'static str,
  tags :        Vec<&'static str,>,
  status :      &'static str,
}

#[component]
pub fn Projects() -> impl IntoView {
  let projects = vec![
    Project {
      title :       "Full-Stack Portfolio",
      description : "Rust-powered portfolio site with Leptos + Axum, showcasing modern full-stack \
                     development with HTMX and Tailwind",
      tags :        vec!["Rust", "Leptos", "Axum", "HTMX"],
      status :      "🚀 Active",
    },
    Project {
      title :       "Data Pipeline System",
      description : "High-performance data pipeline using Rust and Delta Lake for efficient data \
                     ingestion and transformation",
      tags :        vec!["Rust", "Delta Lake", "Apache Spark"],
      status :      "🔨 Building",
    },
    Project {
      title :       "Graph Analytics Platform",
      description : "Analytics platform using Neo4j to model complex relationships in business \
                     data",
      tags :        vec!["Neo4j", "Cypher", "Rust"],
      status :      "🔨 Building",
    },
    Project {
      title :       "CLI Utilities Suite",
      description : "Cross-platform developer productivity tools and system utilities",
      tags :        vec!["Rust", "CLI", "Cross-platform"],
      status :      "💡 Planning",
    },
  ];

  view! {
    <section id="projects" class="projects">
      <h2>"🚀 Projects"</h2>
      <div class="projects__grid">
        {projects.into_iter().map(|p| view! { <ProjectCard project=p /> }).collect_view()}
      </div>
    </section>
  }
}

#[component]
fn ProjectCard(project : Project,) -> impl IntoView {
  view! {
    <article class="project-card">
      <header>
        <h3>{project.title}</h3>
        <span>{project.status}</span>
      </header>
      <p>{project.description}</p>
      <ul role="list">
        {project.tags.into_iter().map(|tag| view! { <li>{tag}</li> }).collect_view()}
      </ul>
    </article>
  }
}
