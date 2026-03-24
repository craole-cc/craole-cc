use super::_prelude::*;

// ── Hooks ─────────────────────────────────────────────────────────────────────

fn use_spotlight_keyboard(set_open : WriteSignal<bool,>,) {
  #[cfg(feature = "hydrate")]
  Effect::new(move |_| {
    let handler = Closure::<dyn Fn(web_sys::KeyboardEvent,),>::wrap(Box::new(
      move |e : web_sys::KeyboardEvent| {
        if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
          e.prevent_default();
          set_open.update(|v| *v = !*v,);
        }
        if e.key() == "Escape" {
          set_open.set(false,);
        }
      },
    )
      as Box<dyn Fn(web_sys::KeyboardEvent,),>,);
    if let Some(win,) = web_sys::window() {
      let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
      let _ = win.add_event_listener_with_callback("keydown", &cb,);
    }
    handler.forget();
  },);
}

fn use_focus_on_open(
  open : ReadSignal<bool,>,
  set_query : WriteSignal<String,>,
  input_ref : NodeRef<LeptosInput,>,
) {
  Effect::new(move |_| {
    if !open.get() {
      return;
    }
    set_query.set(String::new(),);
    #[cfg(feature = "hydrate")]
    if let Some(win,) = web_sys::window() {
      let cb = Closure::<dyn Fn(),>::wrap(Box::new(move || {
        if let Some(el,) = input_ref.get() {
          let _ = el.focus();
        }
      },) as Box<dyn Fn(),>,);
      let _ =
        win.set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), 30,);
      cb.forget();
    }
  },);
}

// ── Sub-components ────────────────────────────────────────────────────────────

#[component]
fn Field(
  #[prop(into)] placeholder : String,
  query : ReadSignal<String,>,
  set_query : WriteSignal<String,>,
  input_ref : NodeRef<LeptosInput,>,
) -> impl IntoView {
  view! {
    <div class="spotlight__field">
      <svg
        class="spotlight__icon"
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <input
        type="search"
        class="spotlight__input"
        placeholder=placeholder
        node_ref=input_ref
        prop:value=move || query.get()
        on:input=move |e| set_query.set(event_target_value(&e))
      />
      <kbd class="spotlight__esc">"Esc"</kbd>
    </div>
  }
}

#[component]
fn Results(
  query : ReadSignal<String,>,
  results : Resource<Result<Vec<Item,>, ServerFnError,>,>,
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  view! {
    <Suspense fallback=|| ()>
      {move || results.get().map(|res| res.ok().map(|items| {
        if items.is_empty() && query.get().len() >= 2 {
          return view! {
            <div class="spotlight__empty"><p>"No results found."</p></div>
          }.into_any();
        }

        let (mut dev, mut art, mut log) = (vec![], vec![], vec![]);
        for item in items {
          match item.kind {
            Kind::Project => dev.push(item),
            Kind::Art    => art.push(item),
            Kind::Log    => log.push(item),
          }
        }

        view! {
          <div class="spotlight__results">
            <ResultGroup label="Dev" class="spotlight__kind--dev" items=dev set_open=set_open />
            <ResultGroup label="Art" class="spotlight__kind--art" items=art set_open=set_open />
            <ResultGroup label="Log" class="spotlight__kind--log" items=log set_open=set_open />
          </div>
        }.into_any()
      }))}
    </Suspense>
  }
}

// ── Root component ────────────────────────────────────────────────────────────

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
      <div class="spotlight" role="dialog" aria-label="Site search">
        <Field
          placeholder=placeholder()
          query=query
          set_query=set_query
          input_ref=input_ref
        />

        {move || (!query.get().is_empty()).then_some(()).is_none().then(||
          context_tags
            .get()
            .and_then(|res| res.ok().flatten())
            .map(|(label, tags)| view! {
              <Tags tags=tags label=label page=current_path() set_open=set_open />
            })
        ).flatten()}

        <Results query=query results=results set_open=set_open />
      </div>
    </Show>
  }
}
