use crate::prelude::{
  icons::{
    menu_close,
    menu_open,
  },
  *,
};

#[component]
fn Hamburger(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <button
      class="site-nav__menu-btn"
      aria-label=move || if open.get() { "Close menu" } else { "Open menu" }
      aria-expanded=move || open.get().to_string()
      aria-controls="primary-nav"
      on:click=move |_| set_open.update(|v| *v = !*v)
    >
      {move || {
        let icon = if open.get() { menu_close::default() } else { menu_open::default() };
        view! { <IconRender icon /> }
      }}
    </button>
  }
}

#[component]
pub fn Controls(open : ReadSignal<bool,>, set_open : WriteSignal<bool,>,) -> impl IntoView {
  view! {
    <div class="site-nav__controls">
      <ThemeSwitcher />
      <Hamburger open set_open />
    </div>
  }
}
