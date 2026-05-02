use {
  super::{filter::Filter, header::Header, mosaic::Mosaic},
  crate::prelude::*,
};

#[component]
pub fn Art() -> impl IntoView {
  let (media, set_media) = signal(Vec::<Media>::new());

  let initial = Resource::new(|| (), |()| async move { list_media().await });

  // Restore scroll position when returning from a detail page
  Effect::new(move |_| {
    if let Some(win) = web_sys::window()
      && let Ok(Some(storage)) = win.session_storage()
      && let Ok(Some(y)) = storage.get_item("art_scroll")
    {
      let _ = storage.remove_item("art_scroll");
      if let Ok(y) = y.parse::<f64>() {
        // Defer scroll until after paint so the mosaic is laid out
        let win_clone = win.clone();
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
          &js_sys::Function::new_no_args(&format!(
            "window.scrollTo({{top: {y}, behavior: 'instant'}})"
          )),
          50,
        );
        drop(win_clone);
      }
    }
  });

  view! {
    <div class="art-page">
      <Header />
      <Filter on_media_change=Callback::new(move |m| set_media.set(m)) />

      <Suspense fallback=move || {
        view! {
          <p class="art-loading readable" aria-busy="true">
            "Loading…"
          </p>
        }
          .into_any()
      }>
        {move || {
          let resolved = initial.get();
          let items = match resolved {
            None => {
              return view! { <p class="art-loading readable">"Loading…"</p> }.into_any();
            }
            Some(Err(e)) => {
              return view! { <p class="art-loading readable">"Error: " {e.to_string()}</p> }
                .into_any();
            }
            Some(Ok(data)) => {
              if media.get_untracked().is_empty() {
                set_media.set(data);
              }
              media.get()
            }
          };
          view! { <Mosaic items=items /> }.into_any()
        }}
      </Suspense>
      <BackToTop />
    </div>
  }
}
