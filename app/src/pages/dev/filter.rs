use super::_prelude::*;

const STATUSES: [(&str, &str,); 4] = [
  ("active", "Active",),
  ("building", "Building",),
  ("planning", "Planning",),
  ("archived", "Archived",),
];

#[component]
pub fn Filter(on_projects_change : Callback<Vec<Project,>,>,) -> impl IntoView {
  let (active_status, set_active_status,) = signal(Option::<String,>::None,);

  let projects = Resource::new(
    move || active_status.get(),
    |s| async move {
      if let Some(status,) = s {
        list_projects_by_status(status,).await
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
      <nav class="dev-filter__statuses" aria-label="Filter by status">
        <button
          class=move || {
            if active_status.get().is_none() {
              "dev-filter__btn dev-filter__btn--active"
            } else {
              "dev-filter__btn"
            }
          }
          on:click=move |_| set_active_status.set(None)
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
                }
              >
                {label}
              </button>
            }
          })
          .collect_view()}
      </nav>
    </div>
  }
}
