use super::_prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Field(
  #[prop(into)] placeholder : String,
  query : ReadSignal<String,>,
  set_query : WriteSignal<String,>,
  input_ref : NodeRef<LeptosInput,>,
) -> impl IntoView {
  view! {
    <div role="search" class="spotlight__field">
      <svg
        class="spotlight__icon"
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
      <input
        type="search"
        class="spotlight__input"
        placeholder=placeholder
        node_ref=input_ref
        prop:value=move || query.get()
        on:input=move |e| set_query.set(event_target_value(&e))
      />
      <kbd class="spotlight__esc">"Esc"</kbd>
    </div>
  }
}
