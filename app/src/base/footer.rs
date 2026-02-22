use crate::prelude::{
  icons::*,
  *,
};

const SOCIALS : &[Icons] = &[
  Icons::Gmail,
  Icons::GitHub,
  Icons::LinkedIn,
  Icons::WhatsApp,
  Icons::Instagram,
  Icons::Facebook,
  Icons::X,
];

const SOCIAL_ICON_SIZE : &str = "w-7 h-7";

#[component]
fn SocialIcon(icon_enum : Icons,) -> impl IntoView {
  let default_icon = icon_enum
        .get()
        .with_source(match icon_enum {
            Icons::WhatsApp  => whatsapp_variants::filled(),
            Icons::GitHub    => github_variants::filled(),
            Icons::Instagram => instagram_variants::filled(),
            Icons::LinkedIn  => linkedin_variants::filled(),
            Icons::Gmail     => gmail_variants::filled(),
            Icons::Facebook  => facebook_variants::filled(),
            Icons::X         => x_variants::filled(),
            _                => icon_enum.get().source,
        })
        //? "fill-muted" is a semantic class from _theme.scss
        .and_class("fill-muted");

  let hover_icon = match icon_enum {
    | Icons::GitHub => github_variants::with_color(github_variants::outlined(),),
    | Icons::X => x_variants::with_color(x_variants::outlined(),),
    | _ => icon_enum.get(),
  };

  view! {
    <a
      href=default_icon.link()
      target="_blank"
      rel="noopener noreferrer"
      aria-label=default_icon.label()
      title=default_icon.tooltip()
      class="inline-flex relative justify-center items-center w-10 h-10 rounded-lg transition-all duration-300 hover:-translate-y-1 group"
    >
      <div class="flex absolute inset-0 justify-center items-center transition-opacity duration-300 group-hover:opacity-0">
        <IconRender icon=default_icon class=SOCIAL_ICON_SIZE />
      </div>
      <div class="flex absolute inset-0 justify-center items-center opacity-0 transition-opacity duration-300 group-hover:opacity-100">
        <IconRender icon=hover_icon class=SOCIAL_ICON_SIZE />
      </div>
    </a>
  }
}

#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <div class="flex flex-wrap gap-2 justify-center">
      {SOCIALS.iter().map(|&icon| view! { <SocialIcon icon_enum=icon /> }).collect::<Vec<_>>()}
    </div>
  }
}

#[component]
pub fn Facets() -> impl IntoView {
  view! {
    <div class="flex gap-4 justify-center items-center text-sm uppercase">
      {FACETS
        .iter()
        .enumerate()
        .map(|(i, facet)| {
          let border = if i < FACETS.len() - 1 { "border-r border-neutral pr-4" } else { "" };
          view! {
            <a
              href=format!("/{}", facet.slug)
              title=facet.description
              class=format!(
                "transition-all duration-300 cursor-pointer hover:scale-110 hover:-translate-y-0.5 hover:font-bold link-primary {border}",
              )
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
    <p class="mb-2 tracking-tight text-md">
      <span>{AUTHOR_FIRSTNAME}</span>
      <span class="text-xl font-semibold">" "{AUTHOR_ALIAS}" "</span>
      <span>{AUTHOR_SURNAME}</span>
    </p>
    <div class="text-xs text-faint">
      <p>
        "Built with " <span class="font-medium text-orange-500 dark:text-orange-300">"Rust"</span>
        " & " <span class="font-medium text-red-500 dark:text-red-300">"Leptos"</span>
      </p>
      <p>"© " {COPYRIGHT_YEAR} " — All rights reserved"</p>
    </div>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="text-center text-muted">
      <Divider />
      <nav class="grid gap-2">
        <Socials />
        <Facets />
      </nav>
      <Divider config=Divider::default_with_dot() />
      <Copyright />
    </footer>
  }
}
