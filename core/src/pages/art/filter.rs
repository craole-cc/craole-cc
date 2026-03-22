use crate::prelude::*;

#[component]
pub fn Filter(on_media_change : Callback<Vec<Media,>,>,) -> impl IntoView {
  let (query, set_query,) = signal(String::new(),);
  let (active_tag, set_active_tag,) = signal(Option::<String,>::None,);

  let tags = Resource::new(|| (), |()| async move { list_media_tags().await },);

  let media = Resource::new(
    move || (query.get(), active_tag.get(),),
    |(q, t,)| async move {
      if !q.is_empty() {
        search_media(q,).await
      } else if let Some(tag,) = t {
        list_media_by_tag(tag,).await
      } else {
        list_media().await
      }
    },
  );

  Effect::new(move |_| {
    if let Some(Ok(m,),) = media.get() {
      on_media_change.run(m,);
    }
  },);

  view! {
    <div class="art-filter readable">
      <div class="art-filter__search">
        <input
          type="search"
          placeholder="Search the gallery…"
          class="art-filter__input"
          on:input=move |e| {
            set_query.set(event_target_value(&e));
            set_active_tag.set(None);
          }
        />
      </div>
      <Suspense fallback=|| ()>
        {move || {
          tags
            .get()
            .map(|res: Result<Vec<String>, ServerFnError>| {
              res
                .ok()
                .map(|t: Vec<String>| {
                  view! {
                    <nav class="art-filter__tags" aria-label="Filter by tag">
                      <button
                        class=move || {
                          if active_tag.get().is_none() && query.get().is_empty() {
                            "art-filter__tag art-filter__tag--active"
                          } else {
                            "art-filter__tag"
                          }
                        }
                        on:click=move |_| {
                          set_active_tag.set(None);
                          set_query.set(String::new());
                        }
                      >
                        "All"
                      </button>
                      {t
                        .into_iter()
                        .map(|tag: String| {
                          let tag_class = tag.clone();
                          let tag_click = tag.clone();
                          let tag_label = tag;
                          view! {
                            <button
                              class=move || {
                                if active_tag.get().as_deref() == Some(&tag_class) {
                                  "art-filter__tag art-filter__tag--active"
                                } else {
                                  "art-filter__tag"
                                }
                              }
                              on:click=move |_| {
                                set_active_tag.set(Some(tag_click.clone()));
                                set_query.set(String::new());
                              }
                            >
                              {tag_label}
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
