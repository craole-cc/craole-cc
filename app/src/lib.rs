mod views;

use {
  leptos::prelude::*,
  leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context},
  leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
  },
  views::*,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/craole-cc.css" />
    <Title text="Craole-CC" />
    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage />
        </Routes>
      </main>
    </Router>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  view! {
    <h1>"Welcome to Craole.CC!"</h1>
    <Test />
  }
}
