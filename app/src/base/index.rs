use crate::prelude::*;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/craole-cc.css" />
    <Title text="Craole-CC" />
    <ThemeProvider>
      <Router>
        <Header />
        <main>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=HomePage />
            <Route path=StaticSegment("dev") view=DevPage />
            <Route path=StaticSegment("art") view=ArtPage />
            <Route path=StaticSegment("log") view=LogPage />
          </Routes>
        </main>
        <Footer />
      </Router>
    </ThemeProvider>
  }
}

pub fn shell(options : LeptosOptions,) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />

        // ? inner_html is required — text nodes inside view! are
        // ? HTML-escaped, so plain string content never executes
        <script inner_html=THEME_INIT />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}
