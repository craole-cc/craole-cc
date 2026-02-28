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
