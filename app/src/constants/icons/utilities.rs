use {
  super::{
    Icon,
    Source,
  },
  crate::_prelude::*,
};

#[component]
pub fn Render(icon : Icon, #[prop(optional, into)] class : Option<String,>,) -> impl IntoView {
  let extra = class.unwrap_or_default();
  let final_class = if extra.is_empty() {
    icon.class.to_string()
  } else {
    format!("{} {}", icon.class, extra)
  };

  match icon.source {
    | Source::Leptos(ico,) => view! {
      <div class=final_class aria-label=icon.label title=icon.tooltip>
        <LeptosIcon icon=ico />
      </div>
    }
    .into_any(),

    | Source::Local(src,) => view! {
      <img
        src=src
        class=final_class
        alt=icon.label
        title=icon.tooltip
        loading="lazy"
        onerror="this.style.display='none'"
      />
    }
    .into_any(),

    | Source::Empty => view! { <span /> }.into_any(),
  }
}
