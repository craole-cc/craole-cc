use leptos::prelude::*;
use leptos_lucide::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
      <section class="py-20 mb-16 text-center">
        <div class="mb-8">
          <div class="p-1 mx-auto mb-6 w-32 h-32 bg-gradient-to-br from-blue-400 to-purple-500 rounded-full">
            <div class="flex justify-center items-center w-full h-full rounded-full bg-slate-800 text-slate-400 dark:bg-slate-900">
              <div class="w-16 h-16">{icon!(User)}</div>
            </div>
          </div>
        </div>

        <h1 class="mb-6 text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500">
          "Your Name"
        </h1>
        <p class="mb-4 text-2xl text-slate-600 dark:text-slate-300">
          "Data Engineer | Backend Developer | TEFL Tutor"
        </p>
        <p class="mx-auto mb-8 max-w-3xl text-lg text-slate-500 dark:text-slate-400">
          "Building robust data pipelines and systems that power analytics"
        </p>

        <div class="flex flex-wrap gap-4 justify-center mt-8">
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Wrench)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Rust"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Database)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"SQL"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Layers)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Delta Lake"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Network)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Neo4j"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Code2)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Python"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(BarChart3)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"Power BI"</span>
          </div>
          <div class="flex gap-2 items-center py-2 px-4 rounded-lg border transition-all hover:border-blue-400 bg-slate-100 border-slate-200 group dark:bg-slate-800/80 dark:border-slate-700 dark:hover:border-blue-500">
            <div class="w-4 h-4 transition-colors group-hover:text-blue-500 text-slate-600 dark:text-slate-400">
              {icon!(Package)}
            </div>
            <span class="text-sm font-medium text-slate-700 dark:text-slate-300">"NixOS"</span>
          </div>
        </div>

        <div class="flex gap-4 justify-center mt-8">
          <a
            href="https://github.com/yourusername"
            target="_blank"
            rel="noopener noreferrer"
            class="flex gap-2 items-center py-3 px-6 text-white rounded-lg border-2 transition-all hover:bg-blue-600 hover:border-blue-600 bg-slate-900 border-slate-700 dark:bg-slate-800"
          >
            <div class="w-5 h-5">{icon!(Github)}</div>
            "GitHub"
          </a>
          <a
            href="https://linkedin.com/in/yourprofile"
            target="_blank"
            rel="noopener noreferrer"
            class="flex gap-2 items-center py-3 px-6 text-white bg-blue-600 rounded-lg transition-all hover:bg-blue-700"
          >
            <div class="w-5 h-5">{icon!(Linkedin)}</div>
            "LinkedIn"
          </a>
        </div>
      </section>
    }
}
