use {
  super::_prelude::*,
  crate::database::posts::{
    list_post_kinds,
    list_posts_by_kind,
    search_posts,
  },
};

#[component]
pub fn Filter(
  on_posts_change : Callback<Vec<PostSummary,>,>,
  on_kinds_change : Callback<Vec<String,>,>,
) -> impl IntoView {
  let (query, set_query,) = signal(String::new(),);
  let (active_kind, set_active_kind,) = signal(Option::<String,>::None,);

  // Load kinds once on mount
  let kinds = Resource::new(|| (), |()| async move { list_post_kinds().await },);

  // Reactive posts — re-fetches when query or kind changes
  let posts = Resource::new(
    move || (query.get(), active_kind.get(),),
    |(q, k,)| async move {
      if !q.is_empty() {
        search_posts(q,).await
      } else if let Some(kind,) = k {
        list_posts_by_kind(kind,).await
      } else {
        list_posts().await
      }
    },
  );

  // Push kinds up to parent when loaded
  Effect::new(move |_| {
    if let Some(Ok(k,),) = kinds.get() {
      on_kinds_change.run(k,);
    }
  },);

  // Push posts up to parent whenever they change
  Effect::new(move |_| {
    if let Some(Ok(p,),) = posts.get() {
      on_posts_change.run(p,);
    }
  },);

  view! {
    <div class="log-filter readable">

      // -- Search ─────────────────────────────────────────────────────────────
      <div class="log-filter__search">
        <input
          type="search"
          placeholder="Search entries…"
          class="log-filter__input"
          on:input=move |e| {
            set_query.set(event_target_value(&e));
            set_active_kind.set(None);
          }
        />
      </div>

      // -- Kind filters ───────────────────────────────────────────────────────
      <Suspense fallback=|| ()>
        {move || {
          kinds
            .get()
            .map(|res| {
              res
                .ok()
                .map(|k| {
                  view! {
                    <nav class="log-filter__kinds" aria-label="Filter by kind">
                      <button
                        class=move || {
                          if active_kind.get().is_none() && query.get().is_empty() {
                            "log-filter__kind log-filter__kind--active"
                          } else {
                            "log-filter__kind"
                          }
                        }
                        on:click=move |_| {
                          set_active_kind.set(None);
                          set_query.set(String::new());
                        }
                      >
                        "All"
                      </button>
                      {k
                        .into_iter()
                        .map(|kind| {
                          let kind_for_class = kind.clone();
                          let kind_for_click = kind.clone();
                          let kind_for_display = kind;
                          view! {
                            <button
                              class=move || {
                                if active_kind.get().as_deref() == Some(&kind_for_class) {
                                  "log-filter__kind log-filter__kind--active"
                                } else {
                                  "log-filter__kind"
                                }
                              }
                              on:click=move |_| {
                                set_active_kind.set(Some(kind_for_click.clone()));
                                set_query.set(String::new());
                              }
                            >
                              {kind_for_display}
                            </button>
                          }
                        })
                        .collect_view()}
                    </nav>
                  }
                })
            })
        }}
      </Suspense>

    </div>
  }
}
