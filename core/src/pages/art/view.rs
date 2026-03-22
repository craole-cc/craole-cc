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

  Effect::new(move |_| {
    if let Some(Ok(m,),) = initial.get() {
      set_media.set(m,);
    }
  },);

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
          let items = media.get();
          view! { <Mosaic items=items /> }.into_any()
        }}
      </Suspense>
    </div>
  }
}
