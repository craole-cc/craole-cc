use crate::prelude::*;

const DATA_TAGS : &[&str] = &[
  "Data",
  "Data Engineering",
  "Business Intelligence",
  "BI",
  "Analytics",
  "Dashboard",
  "Power BI",
  "Tableau",
  "SQL",
  "PostgreSQL",
  "SQLite",
  "Neo4j",
  "Delta Lake",
  "Spark",
  "Databricks",
];

fn is_data_project(project : &Project,) -> bool {
  project.tags.iter().any(|tag| {
    DATA_TAGS
      .iter()
      .any(|data_tag| tag.eq_ignore_ascii_case(data_tag,),)
  },)
}

#[component]
pub fn Data() -> impl IntoView {
  let projects = Resource::new(
    || (),
    |()| async move {
      list_projects().await.map(|projects| {
        projects
          .into_iter()
          .filter(is_data_project,)
          .collect::<Vec<_,>>()
      },)
    },
  );

  view! {
    <div class="dev-page data-page">
      <header class="dev-header readable">
        <span class="dev-header__label">"Business intelligence"</span>
        <h1 class="dev-header__title">"Data"</h1>
        <p class="dev-header__sub">
          "Dashboards, analytics systems, reporting workflows, and data products that turn raw information into decisions."
        </p>
      </header>

      <Suspense fallback=move || {
        view! {
          <p class="dev-loading readable" aria-busy="true">
            "Loading…"
          </p>
        }
          .into_any()
      }>
        {move || match projects.get() {
          | Some(Ok(items,)) if items.is_empty() => view! {
            <div class="dev-empty readable">
              <h2 class="dev-empty__title">"BI case studies are being prepared."</h2>
              <p class="dev-empty__body">
                "Published projects tagged Business Intelligence, BI, Analytics, Dashboard, Power BI, Tableau, SQL, or related data tools will appear here."
              </p>
            </div>
          }
            .into_any(),
          | Some(Ok(items,)) => view! { <crate::pages::dev::Grid items=items /> }.into_any(),
          | Some(Err(error,)) => view! {
            <div class="dev-empty readable">
              <h2 class="dev-empty__title">"Data projects could not load."</h2>
              <p class="dev-empty__body">{error.to_string()}</p>
            </div>
          }
            .into_any(),
          | None => ().into_any(),
        }}
      </Suspense>
      <BackToTop />
    </div>
  }
}
