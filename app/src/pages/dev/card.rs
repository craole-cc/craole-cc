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
  let has_repo = project.repo_url.is_some();
  let has_live = project.live_url.is_some();
  let repo = project.repo_url.clone().unwrap_or_default();
  let live = project.live_url.clone().unwrap_or_default();

  view! {
    <article class="dev-card">
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
                .map(|tag| {
                  view! { <li class="dev-card__tag">{tag.clone()}</li> }
                })
                .collect_view()}
            </ul>
          },
        )
      }}

      // -- Links row
      {if has_repo || has_live {
        Some(
          view! {
            <footer class="dev-card__links">
              {if has_repo {
                Some(
                  view! {
                    <a
                      href=repo
                      target="_blank"
                      rel="noopener noreferrer"
                      class="dev-card__link"
                      aria-label="View source on GitHub"
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        aria-hidden="true"
                      >
                        <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
                      </svg>
                      "Source"
                    </a>
                  },
                )
              } else {
                None
              }}
              {if has_live {
                Some(
                  view! {
                    <a
                      href=live
                      target="_blank"
                      rel="noopener noreferrer"
                      class="dev-card__link dev-card__link--live"
                      aria-label="View live site"
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        aria-hidden="true"
                      >
                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                        <polyline points="15 3 21 3 21 9" />
                        <line x1="10" y1="14" x2="21" y2="3" />
                      </svg>
                      "Live"
                    </a>
                  },
                )
              } else {
                None
              }}
            </footer>
          },
        )
      } else {
        None
      }}
    </article>
  }
}
