use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="py-8 mt-12 text-center border-t border-slate-200 dark:border-slate-700/50">
      <p class="mb-2 text-lg font-medium text-slate-700 dark:text-slate-300">
        "Let's build something impactful together!"
      </p>
      <p class="text-sm text-slate-500 dark:text-slate-500">
        "Built with " <span class="font-semibold text-orange-600 dark:text-orange-400">"Rust"</span>
        " & " <span class="font-semibold text-purple-600 dark:text-purple-400">"Leptos"</span>
        " | © 2026 Craig 'Craole' Cole"
      </p>
    </footer>
  }
}
