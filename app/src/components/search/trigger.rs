use super::_prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Trigger(set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <button
      class="spotlight__trigger"
      aria-label="Search site (Cmd+K)"
      title="Search (⌘K)"
      on:click=move |_| set_open.set(true)
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="11" cy="11" r="8" />
        <line x1="21" y1="21" x2="16.65" y2="16.65" />
      </svg>
      <span class="spotlight__shortcut">"⌘K"</span>
    </button>
  }
}
