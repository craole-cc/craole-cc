use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Social Links from Registry                                ║
//╚═══════════════════════════════════════════════════════════╝

const SOCIALS: &[Icons] = &[
  Icons::Gmail,
  Icons::GitHub,
  Icons::LinkedIn,
  Icons::WhatsApp,
  Icons::Instagram,
  Icons::Facebook,
  Icons::X,
];

//╔═══════════════════════════════════════════════════════════╗
//║ Social Icon Component                                      ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
fn SocialIcon(icon_enum: Icons) -> impl IntoView {
  let variant = icon_enum.variant();
  let mut icon = variant.light;

  // Determine if this icon needs color adjustment for monochrome → color transition
  let needs_color_shift = matches!(icon_enum, Icons::GitHub | Icons::WhatsApp | Icons::X);

  icon.class = if needs_color_shift {
    "w-6 h-6 grayscale group-hover:grayscale-0 \
    brightness-[0.6] dark:brightness-100 \
    group-hover:brightness-100 \
    transition-all duration-300"
  } else {
    "w-6 h-6 grayscale group-hover:grayscale-0 transition-all duration-300"
  };

  view! {
    <a
      href=icon.link
      target="_blank"
      rel="noopener noreferrer"
      aria-label=icon.label
      title=icon.tooltip
      class=format!(
        "inline-flex justify-center items-center p-3 {} rounded-lg \
        transition-all duration-300 hover:shadow-lg hover:-translate-y-1 group",
        NEUTRAL_BG_GHOST,
      )
    >
      <IconRender icon=icon.clone() />
    </a>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Facets                                                    ║
//╚═══════════════════════════════════════════════════════════╝

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

//╔═══════════════════════════════════════════════════════════╗
//║ Copyright                                                 ║
//╚═══════════════════════════════════════════════════════════╝

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

//╔═══════════════════════════════════════════════════════════╗
//║ Footer                                                     ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class=format!("text-center {}", NEUTRAL_TEXT_600)>
      <Divider />
      <nav class="grid gap-4">
        <div class="flex flex-wrap justify-center gap-2">
          {SOCIALS.iter().map(|&icon| view! { <SocialIcon icon_enum=icon /> }).collect::<Vec<_>>()}
        </div>
        <Facets />
      </nav>
      <Divider config=Divider::default_with_dot() />
      <Copyright />
    </footer>
  }
}
