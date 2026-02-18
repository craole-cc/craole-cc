use crate::prelude::{icons::*, *};

const SOCIALS: &[Icons] = &[
  Icons::Gmail,
  Icons::GitHub,
  Icons::LinkedIn,
  Icons::WhatsApp,
  Icons::Instagram,
  Icons::Facebook,
  Icons::X,
];

const SOCIAL_ICON_SIZE: &str = "w-6 h-6";

#[component]
fn SocialIcon(icon_enum: Icons) -> impl IntoView {
  //? Default: neutral-colored variants
  let default_icon = icon_enum
    .get()
    .with_source(match icon_enum {
      | Icons::WhatsApp => whatsapp_variants::filled(),
      | Icons::GitHub => github_variants::filled(),
      | Icons::Instagram => instagram_variants::filled(),
      | Icons::LinkedIn => linkedin_variants::filled(),
      | Icons::Gmail => gmail_variants::filled(),
      | Icons::Facebook => facebook_variants::filled(),
      | Icons::X => x_variants::filled(),
      | _ => icon_enum.get().source,
    })
    .and_class(NEUTRAL_FILL);

  //? Hover: brand-colored variants
  let hover_icon = match icon_enum {
    | Icons::GitHub => github_variants::with_color(github_variants::outlined()),
    | Icons::X => x_variants::with_color(x_variants::outlined()),
    | _ => icon_enum.get(),
  };

  view! {
    <a
      href=default_icon.link()
      target="_blank"
      rel="noopener noreferrer"
      aria-label=default_icon.label()
      title=default_icon.tooltip()
      class="relative inline-flex items-center justify-center w-8 h-8 transition-all duration-300 rounded-lg hover:-translate-y-1 group"
    >
      <div class="absolute inset-0 flex items-center justify-center transition-opacity duration-300 group-hover:opacity-0">
        <IconRender icon=default_icon class=SOCIAL_ICON_SIZE />
      </div>

      <div class="absolute inset-0 flex items-center justify-center transition-opacity duration-300 opacity-0 group-hover:opacity-100 grayscale-0">
        <IconRender icon=hover_icon class=SOCIAL_ICON_SIZE />
      </div>
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
            format!("border-r pr-4 {NEUTRAL_BORDER_400}")
          } else {
            String::new()
          };
          view! {
            <a
              href=format!("/{}", facet.slug)
              class=format!(
                "transition-all duration-300 cursor-pointer \
                hover:scale-110 hover:-translate-y-0.5 hover:font-bold {PRIMARY_HOVER_TEXT} {NEUTRAL_HOVER_BG} {border_class}",
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
    <p class=format!("mb-2 tracking-tight text-md {NEUTRAL_TEXT_700}" )>
      <span>{AUTHOR_FIRSTNAME}</span>
      <span class="text-xl font-semibold">" "{AUTHOR_ALIAS}" "</span>
      <span>{AUTHOR_SURNAME}</span>
    </p>
    <p class="text-xs">
      "Built with " <span class="font-medium text-orange-600 dark:text-orange-400">"Rust"</span>
      " & " <span class="font-medium text-purple-600 dark:text-purple-400">"Leptos"</span>
    </p>
    <p class="text-xs">"© " {COPYRIGHT_YEAR} " — All rights reserved"</p>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class=format!("text-center {NEUTRAL_TEXT_600}" )>
      <Divider />
      <nav class="grid">
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
