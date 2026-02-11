use leptos::prelude::*;

#[component]
pub fn view() -> impl IntoView {
    let tech_stack = vec!["Rust", "SQL", "Delta Lake", "Neo4j", "Python", "Power BI", "NixOS"];

    view! {
      <header class="py-20 mb-16 text-center border-b border-slate-700/50">
        <h1 class="mb-6 text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500">
          "Your Name"
        </h1>
        <p class="mb-4 text-2xl text-slate-300">"Data Engineer | Backend Developer | TEFL Tutor"</p>
        <p class="mx-auto mb-8 max-w-3xl text-lg text-slate-400">
          "Building robust data pipelines and systems that power analytics"
        </p>

        <div class="flex flex-wrap gap-3 justify-center mt-8">
          {tech_stack
            .into_iter()
            .map(|tech| {
              view! {
                <span class="py-2 px-4 text-sm font-medium rounded-lg border transition-all bg-slate-800/80 border-slate-700 text-slate-300 hover:bg-slate-700/80 hover:border-slate-600">
                  {tech}
                </span>
              }
            })
            .collect::<Vec<_>>()}
        </div>
      </header>
    }
}
