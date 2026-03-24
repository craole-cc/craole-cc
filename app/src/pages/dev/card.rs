use super::_prelude::*;

/// Status badge colour class.
fn status_class(status : &str,) -> &'static str {
  match status {
    | "active" => "dev-card__status--active",
    | "building" => "dev-card__status--building",
    | _ => "dev-card__status--planning",
  }
}

/// Human-readable status label.
fn status_label(status : &str,) -> &'static str {
  match status {
    | "active" => "Active",
    | "building" => "Building",
    | "planning" => "Planning",
    | "archived" => "Archived",
    | _ => "Unknown",
  }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Card(project : Project,) -> impl IntoView {
  let href = format!("/dev/{}", project.slug);

  view! {
    <a href=href class="dev-card" aria-label=format!("View {}", project.title)>
      // -- Header: title + status badge
      <header class="dev-card__header">
        <h3 class="dev-card__title">{project.title.clone()}</h3>
        <span class=format!(
          "dev-card__status {}",
          status_class(&project.status),
        )>{status_label(&project.status)}</span>
      </header>

      // -- Description
      <p class="dev-card__desc">{project.description.clone()}</p>

      // -- Tags
      {if project.tags.is_empty() {
        None
      } else {
        Some(
          view! {
            <ul class="dev-card__tags" role="list">
              {project
                .tags
                .iter()
                .map(|tag| view! { <li class="dev-card__tag">{tag.clone()}</li> })
                .collect_view()}
            </ul>
          },
        )
      }}
    </a>
  }
}
