use {
  super::{
    Icon,
    LeptosIcon,
    Source,
  },
  crate::prelude::*,
};

/// Renders an [`Icon`] into the appropriate HTML element.
///
/// - Leptos icon → `<LeptosIcon>`
/// - Local asset  → `<img>`
/// - Empty        → `<span>`
#[component]
pub fn Render(icon: Icon) -> impl IntoView {
  match icon.source {
    Source::Leptos(ico) => view! { <LeptosIcon icon=ico attr:class=icon.class /> }.into_any(),
    Source::Local(src) => view! {
      <img
        src=src
        class=icon.class
        alt=icon.label
        loading="lazy"
        onerror="this.style.display='none'"
      />
    }
    .into_any(),

    Source::Empty => view! { <span /> }.into_any(),
  }
}
