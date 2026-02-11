use leptos::prelude::*;

#[component]
pub fn view() -> impl IntoView {
    view! {
      <footer class="py-8 mt-12 text-center border-t border-slate-700/50">
        <p class="text-slate-400">
          "Built with " <span class="font-semibold text-orange-400">"Rust"</span> " & "
          <span class="font-semibold text-purple-400">"Leptos"</span> " | © 2026"
        </p>
      </footer>
    }
}
