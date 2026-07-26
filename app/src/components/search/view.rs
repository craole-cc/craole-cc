use super::_prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Search() -> impl IntoView {
  let (open, set_open,) = signal(false,);
  let (query, set_query,) = signal(String::new(),);
  let input_ref = NodeRef::<LeptosInput,>::new();

  let location = use_location();
  let current_path = move || location.pathname.get();

  let context_tags = Resource::new(current_path, |p| async move { get_context_tags(p,).await },);

  let results = Resource::new(
    move || query.get(),
    |q| async move {
      if q.len() < 2 {
        Ok(vec![],)
      } else {
        sitewide_search(q,).await
      }
    },
  );

  use_spotlight_keyboard(set_open,);
  use_focus_on_open(open, set_query, input_ref,);

  let placeholder = move || match current_path().as_str() {
    | "/dev" => "Search projects, tags…",
    | "/art" => "Search the gallery…",
    | "/log" => "Search entries…",
    | _ => "Search everything…",
  };

  view! {
    <Trigger set_open=set_open />

    <Show when=move || open.get()>
      <div class="spotlight__backdrop" on:click=move |_| set_open.set(false) />

      <div class="spotlight" role="dialog" aria-modal="true" aria-label="Site search">
        <Field placeholder=placeholder() query=query set_query=set_query input_ref=input_ref />

        // Show context tags only when query is empty
        {move || {
          query
            .get()
            .is_empty()
            .then(|| {
              context_tags
                .get()
                .and_then(|res| res.ok().flatten())
                .map(|(label, tags)| {
                  view! { <Tags tags=tags label=label page=current_path() set_open=set_open /> }
                })
            })
        }}

        <Results query=query results=results set_open=set_open />
      </div>
    </Show>
  }
}
