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

#[component]
fn SocialIcon(icon_enum : Icons,) -> impl IntoView {
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
    },)
    .and_class("fill-muted",);

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
      class="footer__social-link group"
    >
      <span class="footer__social-icon footer__social-icon--default">
        <IconRender icon=default_icon class="footer__social-svg" />
      </span>
      <span class="footer__social-icon footer__social-icon--hover">
        <IconRender icon=hover_icon class="footer__social-svg" />
      </span>
    </a>
  }
}

#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <div class="footer__socials">
      {SOCIALS.iter().map(|&icon| view! { <SocialIcon icon_enum=icon /> }).collect::<Vec<_>>()}
    </div>
  }
}

#[component]
pub fn Facets() -> impl IntoView {
  view! {
    <nav class="footer__facets">
      {FACETS
        .iter()
        .enumerate()
        .map(|(i, facet)| {
          let is_last = i == FACETS.len() - 1;
          view! {
            <a
              href=format!("/{}", facet.slug)
              title=facet.description
              class=if is_last { "footer__facet" } else { "footer__facet footer__facet--divided" }
            >
              {facet.label}
            </a>
          }
        })
        .collect::<Vec<_>>()}
    </nav>
  }
}

#[component]
pub fn Copyright() -> impl IntoView {
  view! {
    <div class="footer__copyright">
      <p class="footer__name">
        <span>{AUTHOR_FIRSTNAME}</span>
        <span class="footer__alias">" "{AUTHOR_ALIAS}" "</span>
        <span>{AUTHOR_SURNAME}</span>
      </p>
      <div class="footer__meta">
        <p>
          "Built with "
          <a
            href="https://www.rust-lang.org"
            target="_blank"
            rel="noopener noreferrer"
            class="footer__tech footer__tech--rust"
          >
            "Rust"
          </a> " & "
          <a
            href="https://leptos.dev"
            target="_blank"
            rel="noopener noreferrer"
            class="footer__tech footer__tech--leptos"
          >
            "Leptos"
          </a>
        </p>
        <p>"© "{COPYRIGHT_YEAR}" — All rights reserved"</p>
      </div>
    </div>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="footer">
      <Divider />
      <div class="footer__inner">
        <Socials />
        <Facets />
      </div>
      <Divider config=Divider::default_with_dot() />
      <div class="footer__bottom">
        <Copyright />
        <ThemeSwitcher />
      </div>
    </footer>
  }
}
