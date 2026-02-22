use leptos::prelude::*;

/// Full-page shell — applies the theme background and text colour.
/// Does NOT constrain width; individual sections handle their own
/// max-width and padding so the hero can be truly full-bleed.
#[component]
pub fn Page(children : Children,) -> impl IntoView {
  view! {

    <div class="min-h-screen bg-(--color-bg) text-(--color-text)">{children()}</div>
  }
}

/// Centred content wrapper for every section that is NOT full-bleed.
/// Use this inside About, Projects, Experience, etc.
/// The hero skips this entirely.
#[component]
pub fn Section(children : Children,) -> impl IntoView {
  view! { <div class="py-8 px-6 mx-auto w-full max-w-6xl">{children()}</div> }
}
