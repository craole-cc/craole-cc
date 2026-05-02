use super::{_prelude::*, archive::Archive, featured::Featured, filter::Filter, header::Header};

#[component]
pub fn Log() -> impl IntoView {
  let (posts, set_posts) = signal(Vec::<PostSummary>::new());

  // Initial load
  let initial = Resource::new(|| (), |()| async move { list_posts().await });

  // Seed posts signal from initial load
  Effect::new(move |_| {
    if let Some(Ok(p)) = initial.get() {
      set_posts.set(p);
    }
  });

  view! {
    <div class="log-page">
      <Header />

      <Filter
        on_posts_change=Callback::new(move |p| set_posts.set(p))
        // kinds handled inside Filter
        on_kinds_change=Callback::new(|_| ())
      />

      <Suspense fallback=move || {
        view! {
          <p class="log-loading readable" aria-busy="true">
            "Loading…"
          </p>
        }
          .into_any()
      }>
        {move || {
          let items = posts.get();
          if items.is_empty() {
            return view! {
              <div class="log-empty readable">
                <h2 class="log-empty__title">"Nothing here yet."</h2>
                <p class="log-empty__body">
                  "Posts, experiments, and things worth writing down will appear here."
                </p>
              </div>
            }
              .into_any();
          }
          let first = items.first().cloned();
          let second = items.get(1).cloned();
          let third = items.get(2).cloned();
          let archive = items.iter().skip(3).cloned().collect::<Vec<_>>();

          view! {
            <div>
              <Featured first=first second=second third=third />
              <Archive posts=archive />
            </div>
          }
            .into_any()
        }}
      </Suspense>
      <BackToTop />
    </div>
  }
}
