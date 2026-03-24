use super::_prelude::*;

const STATUSES : [(&str, &str,); 4] = [
  ("active", "Active",),
  ("building", "Building",),
  ("planning", "Planning",),
  ("archived", "Archived",),
];

fn parse_tag_from_search(search : &str,) -> Option<String,> {
  search
    .strip_prefix("?tag=",)
    .map(|v| v.replace("%20", " ",).replace('+', " ",),)
    .filter(|v| !v.is_empty(),)
}

#[component]
pub fn Filter(on_projects_change : Callback<Vec<Project,>,>,) -> impl IntoView {
  let (active_status, set_active_status,) = signal(Option::<String,>::None,);

  // Initialise tag from URL; also kept reactive via Effect below
  let location = use_location();
  let (active_tag, set_active_tag,) =
    signal(parse_tag_from_search(&location.search.get_untracked(),),);

  // Sync active_tag whenever the URL search string changes (spotlight nav)
  Effect::new(move |_| {
    let search = location.search.get(); // reactive dependency
    if let Some(tag,) = parse_tag_from_search(&search,) {
      if active_tag.get_untracked().as_deref() != Some(&tag,) {
        set_active_status.set(None,); // clear status filter when tag is set
        set_active_tag.set(Some(tag,),);
      }
    }
  },);

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
            set_active_status.set(None,);
            set_active_tag.set(None,);
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
