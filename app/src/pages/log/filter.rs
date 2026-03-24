use {
  super::_prelude::*,
  crate::database::posts::{
    list_post_kinds,
    list_posts_by_kind,
  },
};

fn parse_tag_from_search(search : &str,) -> Option<String,> {
  search
    .strip_prefix("?tag=",)
    .map(|v| v.replace("%20", " ",).replace('+', " ",),)
    .filter(|v| !v.is_empty(),)
}

// ── Sub-component ───────────────────────────────────────────────────────────

#[component]
fn KindFilters(
  #[allow(clippy::needless_pass_by_value)] kinds : Vec<String,>,
  active_kind : ReadSignal<Option<String,>,>,
  set_active_kind : WriteSignal<Option<String,>,>,
) -> impl IntoView {
  view! {
    <nav class="log-filter__kinds" aria-label="Filter by kind">
      <button
        class=move || {
          if active_kind.get().is_none() {
            "log-filter__kind log-filter__kind--active"
          } else {
            "log-filter__kind"
          }
        }
        on:click=move |_| set_active_kind.set(None)
      >
        "All"
      </button>
      {kinds
        .into_iter()
        .map(|kind| {
          let kind_class = kind.clone();
          let kind_click = kind.clone();
          let kind_label = kind;
          view! {
            <button
              class=move || {
                if active_kind.get().as_deref() == Some(&kind_class) {
                  "log-filter__kind log-filter__kind--active"
                } else {
                  "log-filter__kind"
                }
              }
              on:click=move |_| {
                set_active_kind.set(Some(kind_click.clone()));
              }
            >
              {kind_label}
            </button>
          }
        })
        .collect_view()}
    </nav>
  }
}

// ── Main filter ─────────────────────────────────────────────────────────────

#[component]
pub fn Filter(
  on_posts_change : Callback<Vec<PostSummary,>,>,
  on_kinds_change : Callback<Vec<String,>,>,
) -> impl IntoView {
  let location = use_location();
  let (active_kind, set_active_kind,) =
    signal(parse_tag_from_search(&location.search.get_untracked(),),);

  Effect::new(move |_| {
    let search = location.search.get();
    if let Some(kind,) = parse_tag_from_search(&search,)
      && active_kind.get_untracked().as_deref() != Some(&kind,)
    {
      set_active_kind.set(Some(kind,),);
    }
  },);

  let kinds = Resource::new(|| (), |()| async move { list_post_kinds().await },);

  let posts = Resource::new(
    move || active_kind.get(),
    |k| async move {
      if let Some(kind,) = k {
        list_posts_by_kind(kind,).await
      } else {
        list_posts().await
      }
    },
  );

  Effect::new(move |_| {
    if let Some(Ok(k,),) = kinds.get() {
      on_kinds_change.run(k,);
    }
  },);

  Effect::new(move |_| {
    if let Some(Ok(p,),) = posts.get() {
      on_posts_change.run(p,);
    }
  },);

  view! {
    <div class="log-filter readable">
      <Suspense fallback=|| ()>
        {move || {
          kinds
            .get()
            .map(|res| {
              res
                .ok()
                .map(|k| {
                  view! {
                    <KindFilters kinds=k active_kind=active_kind set_active_kind=set_active_kind />
                  }
                })
            })
        }}
      </Suspense>
    </div>
  }
}
