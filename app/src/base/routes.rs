use {
  crate::prelude::*,
  leptos::prelude::*,
  leptos_meta::{
    Stylesheet,
    Title,
    provide_meta_context,
  },
  leptos_router::{
    StaticSegment,
    components::{
      Route,
      Router,
      Routes,
    },
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

// pub fn mount() {
//   mount_to_body(view);
// }

// #[component]
// pub fn App() -> impl IntoView {
//   view! {
//       <h1>"Craole.CC!"</h1>
//     // <ThemeProvider>
//       // <Header />
//       // <Hero/>
//       // <About />
//       // <Projects />
//       // <Experience />
//       // <Contact />
//       // <Footer />
//     // </ThemeProvider>
//   }
// }

// // #[component]
// // pub fn theme_provider(children: Children) -> impl IntoView {
// //   view! {
// //     <div class="min-h-screen bg-white transition-colors dark:bg-slate-900">
// //       <div class="py-12 px-6 mx-auto max-w-7xl">{children()}</div>
// //     </div>
// //   }
// // }
