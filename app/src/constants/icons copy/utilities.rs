use {
  super::{
    Icon,
    LeptosIcon,
    Source,
  },
  crate::prelude::*,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Render – component to actually draw an Icon               ║
//╚═══════════════════════════════════════════════════════════╝
#[component]
pub fn Render(icon: Icon) -> impl IntoView {
  match icon.source {
    Source::Leptos(ico_icon) => view! {
      <div class=icon.class>
        <LeptosIcon icon=ico_icon style=icon.style />
      </div>
    }
    .into_any(),
    Source::Local(src) => view! {
      <img
        src=src
        class=icon.class
        style=icon.style
        alt=""
        loading="lazy"
        onerror="this.style.display='none'"
      />
    }
    .into_any(),
    Source::Empty => view! { <span></span> }.into_any(),
  }
}
