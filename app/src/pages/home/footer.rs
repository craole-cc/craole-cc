use crate::prelude::*;

#[component]
pub fn Facets() -> impl IntoView {
  view! {
    <div class=format!(
      "flex gap-4 justify-center items-center text-sm uppercase {}",
      NEUTRAL_TEXT_600,
    )>
      {FACETS
        .iter()
        .enumerate()
        .map(|(i, facet)| {
          let border_class = if i < FACETS.len() - 1 {
            "border-r border-gray-400 pr-4"
          } else {
            ""
          };
          view! {
            <a
              href=format!("/{}", facet.slug)
              class=format!("transition-colors cursor-help {} {}", NEUTRAL_HOVER_100, border_class)
              title=facet.description
            >
              {facet.label}
            </a>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}

#[component]
pub fn Copyright() -> impl IntoView {
  view! {
    // ~@ Identity
    <p class=format!("mb-2 tracking-tight text-md {}", NEUTRAL_TEXT_700)>
      <span>{AUTHOR_FIRSTNAME}</span>
      <span class="text-xl font-semibold">" "{AUTHOR_ALIAS}" "</span>
      <span>{AUTHOR_SURNAME}</span>
    </p>

    // ~@ Build technolog with orange=Rust brand, purple=Leptos brand
    <p class="text-xs">
      "Built with " <span class="font-medium text-orange-600 dark:text-orange-400">"Rust"</span>
      " & " <span class="font-medium text-purple-600 dark:text-purple-400">"Leptos"</span>
    </p>

    // ~@ Copyright
    <p class="text-xs">"© " {COPYRIGHT_YEAR} " — All rights reserved"</p>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class=format!("text-center {}", NEUTRAL_TEXT_600)>
      <Divider />
      <nav class="grid gap-4">
        <Socials />
        <Facets />
      </nav>
      <Divider config=Divider::default_with_dot() />
      <Copyright />
    </footer>
  }
}
