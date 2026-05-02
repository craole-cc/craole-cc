use crate::prelude::*;

const VISIBLE_TAG_LIMIT: usize = 8;

fn parse_tag_from_search(search: &str) -> Option<String> {
  search
    .strip_prefix("?tag=")
    .map(|v| v.replace("%20", " ").replace('+', " "))
    .filter(|v| !v.is_empty())
}

// ── Sub-component: collapsible tag list with multi-select ───────────────────

#[component]
fn TagFilters(
  #[allow(clippy::needless_pass_by_value)] tags: Vec<String>,
  active_tags: ReadSignal<Vec<String>>,
  set_active_tags: WriteSignal<Vec<String>>,
) -> impl IntoView {
  let (expanded, set_expanded) = signal(false);
  let tag_count = tags.len();
  let needs_toggle = tag_count > VISIBLE_TAG_LIMIT;

  view! {
    <nav class="art-filter__tags" aria-label="Filter by tag">
      <button
        class=move || {
          if active_tags.get().is_empty() {
            "art-filter__tag art-filter__tag--active"
          } else {
            "art-filter__tag"
          }
        }
        on:click=move |_| set_active_tags.set(vec![])
      >
        "All"
      </button>
      {tags
        .into_iter()
        .enumerate()
        .map(|(i, tag)| {
          let tag_class = tag.clone();
          let tag_click = tag.clone();
          view! {
            <button
              class=move || {
                let hidden = !expanded.get() && i >= VISIBLE_TAG_LIMIT;
                let active = active_tags.get().contains(&tag_class);
                match (active, hidden) {
                  (true, false) => "art-filter__tag art-filter__tag--active",
                  (false, false) => "art-filter__tag",
                  (true, true) => "art-filter__tag art-filter__tag--active art-filter__tag--hidden",
                  (false, true) => "art-filter__tag art-filter__tag--hidden",
                }
              }
              on:click=move |_| {
                let mut current = active_tags.get();
                if let Some(pos) = current.iter().position(|t| t == &tag_click) {
                  current.remove(pos);
                } else {
                  current.push(tag_click.clone());
                }
                set_active_tags.set(current);
              }
            >
              {tag}
            </button>
          }
        })
        .collect_view()}

      {if needs_toggle {
        Some(
          view! {
            <button class="art-filter__toggle" on:click=move |_| set_expanded.update(|v| *v = !*v)>
              {move || if expanded.get() { "Show less" } else { "Show more" }}
            </button>
          },
        )
      } else {
        None
      }}
    </nav>
  }
}

// ── Main filter component ───────────────────────────────────────────────────

#[component]
pub fn Filter(on_media_change: Callback<Vec<Media>>) -> impl IntoView {
  // Read initial tag from URL (works on SSR too via #[cfg] guard in original)
  let location = use_location();
  let initial = parse_tag_from_search(&location.search.get_untracked())
    .map(|t| vec![t])
    .unwrap_or_default();
  let (active_tags, set_active_tags) = signal(initial);

  // Reactively sync when the URL search changes (e.g. spotlight navigation)
  Effect::new(move |_| {
    let search = location.search.get(); // reactive dependency
    if let Some(tag) = parse_tag_from_search(&search) {
      // Only overwrite if this tag isn't already selected
      if !active_tags.get_untracked().contains(&tag) {
        set_active_tags.set(vec![tag]);
      }
    }
  });

  let tags = Resource::new(|| (), |()| async move { list_media_tags().await });

  let media = Resource::new(
    move || active_tags.get(),
    |tags| async move {
      if tags.is_empty() {
        list_media().await
      } else if tags.len() == 1 {
        list_media_by_tag(tags.into_iter().next().unwrap()).await
      } else {
        list_media_by_tags(tags.join(",")).await
      }
    },
  );

  Effect::new(move |_| {
    if let Some(Ok(m)) = media.get() {
      on_media_change.run(m);
    }
  });

  view! {
    <div class="art-filter readable">
      <Suspense fallback=|| ()>
        {move || {
          tags
            .get()
            .map(|res| {
              res
                .ok()
                .map(|t| {
                  view! {
                    <TagFilters tags=t active_tags=active_tags set_active_tags=set_active_tags />
                  }
                })
            })
        }}
      </Suspense>
    </div>
  }
}
