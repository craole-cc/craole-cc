use super::_prelude::*;

// -- Shared row type + mapper ────────────────────────────────────────

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow,)]
struct ProjectRow {
  id :          i64,
  title :       String,
  description : String,
  status :      String,
  repo_url :    Option<String,>,
  live_url :    Option<String,>,
  featured :    i64,
  sort_order :  i64,
  created_at :  String,
  tags :        String,
}

#[cfg(feature = "ssr")]
fn row_to_project(r : ProjectRow,) -> Project {
  Project {
    id :          r.id,
    title :       r.title,
    description : r.description,
    status :      r.status,
    repo_url :    r.repo_url,
    live_url :    r.live_url,
    featured :    r.featured != 0,
    sort_order :  r.sort_order,
    created_at :  r.created_at,
    tags :        if r.tags.is_empty() {
      vec![]
    } else {
      r.tags.split(',',).map(str::to_string,).collect()
    },
  }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow,)]
struct TagRow {
  tag : String,
}

// -- Server functions ────────────────────────────────────────────────

#[server(ListProjectTags)]
pub async fn list_project_tags() -> Result<Vec<String,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(TagRow, "sql/projects/list_tags.sql")
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(|r| r.tag,).collect(),)
}

#[server(ListProjectsByStatus)]
pub async fn list_projects_by_status(status : String,) -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/list_by_status.sql", status)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

#[server(ListProjectsByTag)]
pub async fn list_projects_by_tag(tag : String,) -> Result<Vec<Project,>, ServerFnError,> {
  let pool = expect_context::<SqlitePool,>();
  let rows = query_file_as!(ProjectRow, "sql/projects/list_by_tag.sql", tag)
    .fetch_all(&pool,)
    .await
    .map_err(|e| ServerFnError::new(e.to_string(),),)?;
  Ok(rows.into_iter().map(row_to_project,).collect(),)
}

// -- Sub-components ──────────────────────────────────────────────────

const STATUSES : [(&str, &str,); 4] = [
  ("active", "Active",),
  ("building", "Building",),
  ("planning", "Planning",),
  ("archived", "Archived",),
];

#[component]
fn StatusFilters(
  active_status : ReadSignal<Option<String,>,>,
  active_tag : ReadSignal<Option<String,>,>,
  set_active_status : WriteSignal<Option<String,>,>,
  set_active_tag : WriteSignal<Option<String,>,>,
) -> impl IntoView {
  view! {
    <nav class="dev-filter__statuses" aria-label="Filter by status">
      <button
        class=move || {
          if active_status.get().is_none() && active_tag.get().is_none() {
            "dev-filter__btn dev-filter__btn--active"
          } else {
            "dev-filter__btn"
          }
        }
        on:click=move |_| {
          set_active_status.set(None);
          set_active_tag.set(None);
        }
      >
        "All"
      </button>
      {STATUSES
        .into_iter()
        .map(|(key, label)| {
          let key_class = key.to_string();
          let key_click = key.to_string();
          view! {
            <button
              class=move || {
                if active_status.get().as_deref() == Some(&key_class) {
                  "dev-filter__btn dev-filter__btn--active"
                } else {
                  "dev-filter__btn"
                }
              }
              on:click=move |_| {
                set_active_status.set(Some(key_click.clone()));
                set_active_tag.set(None);
              }
            >
              {label}
            </button>
          }
        })
        .collect_view()}
    </nav>
  }
}

#[component]
fn TagFilters(
  tags : Vec<String,>,
  active_tag : ReadSignal<Option<String,>,>,
  set_active_status : WriteSignal<Option<String,>,>,
  set_active_tag : WriteSignal<Option<String,>,>,
) -> impl IntoView {
  view! {
    <nav class="dev-filter__tags" aria-label="Filter by technology">
      {tags
        .into_iter()
        .map(|tag| {
          let tag_class = tag.clone();
          let tag_click = tag.clone();
          view! {
            <button
              class=move || {
                if active_tag.get().as_deref() == Some(&tag_class) {
                  "dev-filter__tag dev-filter__tag--active"
                } else {
                  "dev-filter__tag"
                }
              }
              on:click=move |_| {
                set_active_tag.set(Some(tag_click.clone()));
                set_active_status.set(None);
              }
            >
              {tag}
            </button>
          }
        })
        .collect_view()}
    </nav>
  }
}

// -- Main filter component ───────────────────────────────────────────

#[component]
pub fn Filter(on_projects_change : Callback<Vec<Project,>,>,) -> impl IntoView {
  let (active_status, set_active_status,) = signal(Option::<String,>::None,);
  let (active_tag, set_active_tag,) = signal(Option::<String,>::None,);

  let tags = Resource::new(|| (), |()| async move { list_project_tags().await },);

  let projects = Resource::new(
    move || (active_status.get(), active_tag.get(),),
    |(s, t,)| async move {
      if let Some(status,) = s {
        list_projects_by_status(status,).await
      } else if let Some(tag,) = t {
        list_projects_by_tag(tag,).await
      } else {
        list_projects().await
      }
    },
  );

  Effect::new(move |_| {
    if let Some(Ok(p,),) = projects.get() {
      on_projects_change.run(p,);
    }
  },);

  view! {
    <div class="dev-filter readable">
      <StatusFilters
        active_status=active_status
        active_tag=active_tag
        set_active_status=set_active_status
        set_active_tag=set_active_tag
      />
      <Suspense fallback=|| ()>
        {move || {
          tags
            .get()
            .map(|res| {
              res
                .ok()
                .map(|t| {
                  view! {
                    <TagFilters
                      tags=t
                      active_tag=active_tag
                      set_active_status=set_active_status
                      set_active_tag=set_active_tag
                    />
                  }
                })
            })
        }}
      </Suspense>
    </div>
  }
}
