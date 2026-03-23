use crate::prelude::*;

// ── Search result types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct SearchResult {
  pub title :    String,
  pub subtitle : String,
  pub kind :     SearchKind,
  pub url :      String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq,)]
pub enum SearchKind {
  Project,
  Art,
  Log,
}

// ── Sitewide search server function ─────────────────────────────────────────

#[server(SitewideSearch)]
pub async fn sitewide_search(query : String,) -> Result<Vec<SearchResult,>, ServerFnError,> {
  let fts_query = query
    .split_whitespace()
    .filter(|w| !w.is_empty(),)
    .map(|w| format!("{w}*",),)
    .collect::<Vec<_,>>()
    .join(" ",);

  if fts_query.is_empty() {
    return Ok(vec![],);
  }

  let mut results = Vec::new();

  if let Ok(projects,) = search_projects(query.clone(),).await {
    results.extend(projects.into_iter().take(5,).map(|p| SearchResult {
      title :    p.title,
      subtitle : p.description,
      kind :     SearchKind::Project,
      url :      "/dev".to_string(),
    },),);
  }

  if let Ok(media,) = search_media(query.clone(),).await {
    results.extend(media.into_iter().take(5,).map(|m| SearchResult {
      title :    m.title,
      subtitle : m.caption.unwrap_or_default(),
      kind :     SearchKind::Art,
      url :      format!("/art/{}", m.slug),
    },),);
  }

  if let Ok(posts,) = search_posts(query,).await {
    results.extend(posts.into_iter().take(5,).map(|p| SearchResult {
      title :    p.title,
      subtitle : p.excerpt.unwrap_or_default(),
      kind :     SearchKind::Log,
      url :      format!("/log/{}", p.slug),
    },),);
  }

  Ok(results,)
}

// ── Sub-component: tag chips shown when on /dev ─────────────────────────────

#[component]
fn SpotlightTags(
  #[allow(clippy::needless_pass_by_value)]
  tags : Vec<String,>,
  set_query : WriteSignal<String,>,
) -> impl IntoView {
  view! {
    <div class="spotlight__tags">
      {tags
        .into_iter()
        .map(|tag| {
          let tag_click = tag.clone();
          view! {
            <button
              class="spotlight__chip"
              on:click=move |_| set_query.set(tag_click.clone())
            >
              {tag}
            </button>
          }
        })
        .collect_view()}
    </div>
  }
}

// ── Spotlight component ─────────────────────────────────────────────────────

#[component]
pub fn Spotlight() -> impl IntoView {
  let (open, set_open,) = signal(false,);
  let (query, set_query,) = signal(String::new(),);

  let location = use_location();
  let is_dev = move || location.pathname.get() == "/dev";

  // Load project tags when on /dev
  let dev_tags = Resource::new(
    move || is_dev(),
    |on_dev| async move {
      if on_dev { list_project_tags().await.ok() } else { None }
    },
  );

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

  // Listen for Cmd+K / Ctrl+K globally
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
    ) as Box<dyn Fn(web_sys::KeyboardEvent,),>,);
    if let Some(win,) = web_sys::window() {
      let cb = handler.as_ref().unchecked_ref::<js_sys::Function>().clone();
      let _ = win.add_event_listener_with_callback("keydown", &cb,);
    }
    handler.forget();
  },);

  // Reset query when opening
  Effect::new(move |_| {
    if open.get() {
      set_query.set(String::new(),);
    }
  },);

  view! {
    // -- Trigger button (sits in the nav bar)
    <button
      class="spotlight__trigger"
      aria-label="Search site (Cmd+K)"
      title="Search (⌘K)"
      on:click=move |_| set_open.set(true)
    >
      <svg
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
      <span class="spotlight__shortcut">"⌘K"</span>
    </button>

    // -- Overlay
    <Show when=move || open.get()>
      <div
        class="spotlight__backdrop"
        on:click=move |_| set_open.set(false)
      />
      <div class="spotlight" role="dialog" aria-label="Site search">
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
            placeholder=move || {
              if is_dev() { "Search projects, tags…" } else { "Search everything…" }
            }
            autofocus=true
            prop:value=move || query.get()
            on:input=move |e| set_query.set(event_target_value(&e))
          />
          <kbd class="spotlight__esc">"Esc"</kbd>
        </div>

        // -- Dev tag chips (shown when on /dev and query is empty)
        {move || {
          if is_dev() && query.get().is_empty() {
            dev_tags.get().flatten().map(|tags| {
              view! {
                <SpotlightTags tags=tags set_query=set_query />
              }
            })
          } else {
            None
          }
        }}

        // -- Search results
        <Suspense fallback=|| ()>
          {move || {
            results.get().map(|res| {
              res.ok().map(|items| {
                if items.is_empty() && query.get().len() >= 2 {
                  return view! {
                    <div class="spotlight__empty">
                      <p>"No results found."</p>
                    </div>
                  }
                  .into_any();
                }

                let mut dev_items = Vec::new();
                let mut art_items = Vec::new();
                let mut log_items = Vec::new();
                for item in &items {
                  match item.kind {
                    | SearchKind::Project => dev_items.push(item.clone(),),
                    | SearchKind::Art => art_items.push(item.clone(),),
                    | SearchKind::Log => log_items.push(item.clone(),),
                  }
                }

                let set_open_c = set_open;

                view! {
                  <div class="spotlight__results">
                    {render_group("Dev", "spotlight__kind--dev", dev_items, set_open_c)}
                    {render_group("Art", "spotlight__kind--art", art_items, set_open_c)}
                    {render_group("Log", "spotlight__kind--log", log_items, set_open_c)}
                  </div>
                }
                .into_any()
              })
            })
          }}
        </Suspense>
      </div>
    </Show>
  }
}

fn render_group(
  label : &'static str,
  class : &'static str,
  items : Vec<SearchResult,>,
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  if items.is_empty() {
    return None;
  }
  Some(view! {
    <div class="spotlight__group">
      <h3 class=format!("spotlight__group-label {class}")>{label}</h3>
      <ul class="spotlight__list" role="list">
        {items
          .into_iter()
          .map(|item| {
            let url = item.url.clone();
            view! {
              <li>
                <a
                  href=url
                  class="spotlight__result"
                  on:click=move |_| set_open.set(false)
                >
                  <span class="spotlight__result-title">{item.title.clone()}</span>
                  <span class="spotlight__result-sub">{item.subtitle.clone()}</span>
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </div>
  })
}
