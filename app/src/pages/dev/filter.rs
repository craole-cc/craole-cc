use super::_prelude::*;

// ── Sub-components ──────────────────────────────────────────────────────────

const STATUSES: [(&str, &str,); 4] = [
  ("active", "Active",),
  ("building", "Building",),
  ("planning", "Planning",),
  ("archived", "Archived",),
];

#[component]
fn StatusFilters(
  active_status : ReadSignal<Option<String,>,>,
  active_tag : ReadSignal<Option<String,>,>,
  query : ReadSignal<String,>,
  set_active_status : WriteSignal<Option<String,>,>,
  set_active_tag : WriteSignal<Option<String,>,>,
  set_query : WriteSignal<String,>,
) -> impl IntoView {
  view! {
    <nav class="dev-filter__statuses" aria-label="Filter by status">
      <button
        class=move || {
          if active_status.get().is_none() && active_tag.get().is_none() && query.get().is_empty() {
            "dev-filter__btn dev-filter__btn--active"
          } else {
            "dev-filter__btn"
          }
        }
        on:click=move |_| {
          set_active_status.set(None,);
          set_active_tag.set(None,);
          set_query.set(String::new(),);
        }
      >
        "All"
      </button>
      {STATUSES
        .into_iter()
        .map(|(key, label,)| {
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
                set_active_status.set(Some(key_click.clone(),),);
                set_active_tag.set(None,);
                set_query.set(String::new(),);
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
  #[allow(clippy::needless_pass_by_value)]
  tags : Vec<String,>,
  active_tag : ReadSignal<Option<String,>,>,
  set_active_status : WriteSignal<Option<String,>,>,
  set_active_tag : WriteSignal<Option<String,>,>,
  set_query : WriteSignal<String,>,
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
                set_active_tag.set(Some(tag_click.clone(),),);
                set_active_status.set(None,);
                set_query.set(String::new(),);
              }
            >
              {tag.clone()}
            </button>
          }
        })
        .collect_view()}
    </nav>
  }
}

// ── Main filter component ───────────────────────────────────────────────────

#[component]
pub fn Filter(on_projects_change : Callback<Vec<Project,>,>,) -> impl IntoView {
  let (query, set_query,) = signal(String::new(),);
  let (active_status, set_active_status,) = signal(Option::<String,>::None,);
  let (active_tag, set_active_tag,) = signal(Option::<String,>::None,);

  let tags = Resource::new(|| (), |()| async move { list_project_tags().await },);

  let projects = Resource::new(
    move || (query.get(), active_status.get(), active_tag.get(),),
    |(q, s, t,)| async move {
      if !q.is_empty() {
        search_projects(q,).await
      } else if let Some(status,) = s {
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
      // -- Search
      <div class="dev-filter__search">
        <input
          type="search"
          placeholder="Search projects…"
          class="dev-filter__input"
          on:input=move |e| {
            set_query.set(event_target_value(&e,),);
            set_active_status.set(None,);
            set_active_tag.set(None,);
          }
        />
      </div>

      // -- Status filters
      <StatusFilters
        active_status=active_status
        active_tag=active_tag
        query=query
        set_active_status=set_active_status
        set_active_tag=set_active_tag
        set_query=set_query
      />

      // -- Tag filters
      <Suspense fallback=|| ()>
        {move || {
          tags.get().map(|res| {
            res.ok().map(|t| {
              view! {
                <TagFilters
                  tags=t
                  active_tag=active_tag
                  set_active_status=set_active_status
                  set_active_tag=set_active_tag
                  set_query=set_query
                />
              }
            })
          })
        }}
      </Suspense>
    </div>
  }
}
