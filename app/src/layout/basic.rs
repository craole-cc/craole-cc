use leptos::prelude::*;

#[component]
pub fn Basic(children : Children,) -> impl IntoView {
  view! {
    <div class="min-h-screen via-blue-50 to-purple-50 text-slate-900 bg-linear-to-br from-slate-50 dark:from-slate-900 dark:via-slate-900 dark:to-slate-800 dark:text-slate-100">
      <div class="container py-8 px-6 mx-auto max-w-6xl">{children()}</div>
    </div>
  }
}
