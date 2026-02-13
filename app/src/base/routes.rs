use {
  crate::prelude::*,
  leptos::prelude::*,
  leptos_meta::{Stylesheet, Title, provide_meta_context},
  leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
  },
};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/craole-cc.css" />
    <Title text="Craole-CC" />
    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=Home />
        </Routes>
      </main>
    </Router>
  }
}
