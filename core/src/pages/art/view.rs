use {
  super::{
    filter::Filter,
    header::Header,
    mosaic::Mosaic,
  },
  crate::prelude::*,
};

#[component]
pub fn Art() -> impl IntoView {
  let (media, set_media,) = signal(Vec::<Media,>::new(),);

  let initial = Resource::new(|| (), |()| async move { list_media().await },);

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
              return // Read the resource inside Suspense so it actually suspends
              view! { <p class="art-loading readable">"Loading…"</p> }
                .into_any();
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
          // Seed the signal on first load
          view! { <Mosaic items=items /> }
            .into_any()
        }}
      </Suspense>
    </div>
  }
}
