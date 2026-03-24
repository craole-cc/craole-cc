use super::_prelude::*;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn status_class(status : &str,) -> &'static str {
  match status {
    | "active" => "dev-detail__status--active",
    | "building" => "dev-detail__status--building",
    | _ => "dev-detail__status--planning",
  }
}

fn status_label(status : &str,) -> &'static str {
  match status {
    | "active" => "Active",
    | "building" => "Building",
    | "planning" => "Planning",
    | "archived" => "Archived",
    | _ => "Unknown",
  }
}

// ── Sub-components ────────────────────────────────────────────────────────────

#[component]
#[allow(clippy::needless_pass_by_value)]
fn TechSection(icons : Vec<Icon,>,) -> impl IntoView {
  if icons.is_empty() {
    return None;
  }
  Some(view! {
    <section class="dev-detail__tech">
      <h2 class="dev-detail__section-title">"Tech Stack"</h2>
      <ul class="dev-detail__tech-list" role="list">
        {icons
          .into_iter()
          .map(|icon| {
            let brand_style = icon
              .class()
              .split_whitespace()
              .find(|c| c.starts_with("brand-"))
              .map(|c| format!("--badge-brand:var(--{c})"))
              .unwrap_or_default();
            view! {
              <li>
                <a
                  href=icon.link()
                  target="_blank"
                  rel="noopener noreferrer"
                  aria-label=icon.label()
                  title=icon.tooltip()
                  class="tech-badge"
                  style=brand_style
                >
                  <IconRender icon />
                  <span>{icon.label()}</span>
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </section>
  },)
}

#[component]
fn Links(repo_url : Option<String,>, live_url : Option<String,>,) -> impl IntoView {
  view! {
    <div class="dev-detail__links">
      {repo_url
        .map(|url| {
          view! {
            <a href=url target="_blank" rel="noopener noreferrer" class="dev-detail__link">
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
          }
        })}
      {live_url
        .map(|url| {
          view! {
            <a
              href=url
              target="_blank"
              rel="noopener noreferrer"
              class="dev-detail__link dev-detail__link--live"
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
          }
        })}
    </div>
  }
}

#[component]
fn Screenshots(#[allow(clippy::needless_pass_by_value)] shots : Vec<String,>,) -> impl IntoView {
  if shots.is_empty() {
    return None;
  }
  Some(view! {
    <section class="dev-detail__gallery">
      <h2 class="dev-detail__section-title">"Screenshots"</h2>
      <div class="dev-detail__screenshots">
        {shots
          .into_iter()
          .map(|url| {
            view! {
              <figure class="dev-detail__screenshot">
                <img src=url alt="Project screenshot" loading="lazy" decoding="async" />
              </figure>
            }
          })
          .collect_view()}
      </div>
    </section>
  },)
}

#[component]
#[allow(clippy::needless_pass_by_value)]
fn Content(p : ProjectDetail,) -> impl IntoView {
  let icons = p.icons();

  view! {
    <article class="dev-detail readable">
      <nav class="dev-detail__nav" aria-label="Breadcrumb">
        <a class="dev-detail__back" href="/dev">
          "← Dev"
        </a>
      </nav>

      <header class="dev-detail__header">
        <div class="dev-detail__title-row">
          <h1 class="dev-detail__title">{p.title.clone()}</h1>
          <span class=format!(
            "dev-detail__status {}",
            status_class(&p.status),
          )>{status_label(&p.status)}</span>
        </div>
        <p class="dev-detail__desc">{p.description.clone()}</p>
      </header>

      <TechSection icons />
      <Links repo_url=p.repo_url live_url=p.live_url />
      <Screenshots shots=p.screenshots />

      {p
        .readme_html
        .map(|html| {
          view! {
            <section class="dev-detail__readme">
              <h2 class="dev-detail__section-title">"README"</h2>
              <div class="dev-detail__readme-body markdown" inner_html=html />
            </section>
          }
        })}

      <footer class="dev-detail__footer">
        <a class="dev-detail__back" href="/dev">
          "← Back to Dev"
        </a>
      </footer>
    </article>
  }
}

// ── Root component ────────────────────────────────────────────────────────────

#[component]
pub fn Detail() -> impl IntoView {
  let params = use_params_map();
  let slug = move || params.with(|p| p.get("slug",).unwrap_or_default(),);

  let project = Resource::new(slug, |s| async move {
    if s.is_empty() {
      return Ok(None,);
    }
    get_project_by_slug(s,).await
  },);

  view! {
    <Suspense fallback=move || {
      view! {
        <p class="dev-loading readable" aria-busy="true">
          "Loading project…"
        </p>
      }
        .into_any()
    }>
      {move || {
        project
          .get()
          .map(|res| match res {
            Ok(Some(p)) => view! { <Content p /> }.into_any(),
            Ok(None) => {
              view! {
                <div class="dev-empty readable">
                  <h2>"Project not found."</h2>
                  <a href="/dev">"← Back to Dev"</a>
                </div>
              }
                .into_any()
            }
            Err(e) => {
              view! {
                <div class="dev-empty readable">
                  <h2>"Something went wrong."</h2>
                  <p>{e.to_string()}</p>
                </div>
              }
                .into_any()
            }
          })
          .into_any()
      }}
    </Suspense>
  }
}
