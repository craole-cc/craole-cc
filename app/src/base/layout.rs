use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
  view! {
    <div class="min-h-screen text-slate-900 bg-linear-to-br from-slate-50 via-blue-50 to-purple-50 dark:from-slate-900 dark:via-slate-900 dark:to-slate-800 dark:text-slate-100">
      <div class="container max-w-6xl px-6 py-8 mx-auto">
        {children()}
      </div>
    </div>
  }
}
