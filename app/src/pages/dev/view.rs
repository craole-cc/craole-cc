use {
  super::{
    filter::Filter,
    grid::Grid,
    header::Header,
  },
  crate::prelude::*,
};

#[component]
pub fn Dev() -> impl IntoView {
  let (projects, set_projects,) = signal(Vec::<Project,>::new(),);

  let initial = Resource::new(|| (), |()| async move { list_projects().await },);

  // Seed projects signal from initial load
  Effect::new(move |_| {
    if let Some(Ok(p,),) = initial.get() {
      set_projects.set(p,);
    }
  },);

  view! {
    <div class="dev-page">
      <Header />
      <Filter on_projects_change=Callback::new(move |p| set_projects.set(p)) />

      <Suspense fallback=move || {
        view! {
          <p class="dev-loading readable" aria-busy="true">
            "Loading…"
          </p>
        }
          .into_any()
      }>
        {move || {
          let items = projects.get();
          view! { <Grid items=items /> }.into_any()
        }}
      </Suspense>
      <BackToTop />
    </div>
  }
}
