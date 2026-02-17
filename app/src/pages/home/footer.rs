use crate::prelude::*;

const SOCIALS: &[Icons] = &[
  Icons::Gmail,
  Icons::GitHub,
  Icons::LinkedIn,
  Icons::WhatsApp,
  Icons::Instagram,
  Icons::Facebook,
  Icons::X,
];

#[component]
fn SocialIcon(icon_enum: Icons) -> impl IntoView {
  let icon = icon_enum.get();

  // Special case for X icon
  let size_class = if matches!(icon_enum, Icons::X) {
    "w-5 h-5"
  } else {
    "w-6 h-6"
  };

  let styled_icon = icon.with_class(size_class);

  view! {
    <a
      href=icon.link()
      target="_blank"
      rel="noopener noreferrer"
      aria-label=icon.label()
      title=icon.tooltip()
      class=format!(
        "inline-flex justify-center items-center p-3 {} rounded-lg \
        transition-all duration-300 hover:shadow-lg hover:-translate-y-1 \
        grayscale hover:grayscale-0 opacity-70 hover:opacity-100",
        NEUTRAL_BG_GHOST,
      )
    >
      <IconRender icon=styled_icon />
    </a>
  }
}

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
              class=format!(
                "transition-colors cursor-help {} {}",
                NEUTRAL_HOVER_100,
                border_class
              )
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
    <p class=format!("mb-2 tracking-tight text-md {}", NEUTRAL_TEXT_700)>
      <span>{AUTHOR_FIRSTNAME}</span>
      <span class="text-xl font-semibold">" "{AUTHOR_ALIAS}" "</span>
      <span>{AUTHOR_SURNAME}</span>
    </p>
    <p class="text-xs">
      "Built with "
      <span class="font-medium text-orange-600 dark:text-orange-400">"Rust"</span>
      " & "
      <span class="font-medium text-purple-600 dark:text-purple-400">"Leptos"</span>
    </p>
    <p class="text-xs">"© " {COPYRIGHT_YEAR} " — All rights reserved"</p>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class=format!("text-center {}", NEUTRAL_TEXT_600)>
      <Divider />
      <nav class="grid gap-4">
        <div class="flex flex-wrap gap-2 justify-center">
          {SOCIALS.iter().map(|&icon| view! { <SocialIcon icon_enum=icon /> }).collect::<Vec<_>>()}
        </div>
        <Facets />
      </nav>
      <Divider config=Divider::default_with_dot() />
      <Copyright />
    </footer>
  }
}
