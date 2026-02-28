use crate::prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Social Icons                                              ║
//╚═══════════════════════════════════════════════════════════╝

struct SocialEntry {
  hover :   Icon,
  default : Icon,
}

fn socials() -> Vec<SocialEntry,> {
  vec![
    SocialEntry {
      default : gmail::filled(),
      hover :   gmail::local(),
    },
    SocialEntry {
      default : github::filled(),
      hover :   github::filled(),
    },
    SocialEntry {
      default : linkedin::filled(),
      hover :   linkedin::local(),
    },
    SocialEntry {
      default : whatsapp::filled(),
      hover :   whatsapp::local(),
    },
    SocialEntry {
      default : instagram::filled(),
      hover :   instagram::local(),
    },
    SocialEntry {
      default : facebook::filled(),
      hover :   facebook::local(),
    },
    SocialEntry {
      default : x::filled(),
      hover :   x::filled(),
    },
  ]
}

#[component]
fn SocialIcon(default : Icon, hover : Icon,) -> impl IntoView {
  view! {
    <a
      href=default.link()
      target="_blank"
      rel="noopener noreferrer"
      aria-label=default.label()
      title=default.tooltip()
      class="footer__social-link group"
    >
      <span class="footer__social-icon footer__social-icon--default">
        <IconRender icon=default class="footer__social-svg color-muted" />
      </span>
      <span class="footer__social-icon footer__social-icon--hover">
        <IconRender icon=hover class="footer__social-svg" />
      </span>
    </a>
  }
}

#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <div class="footer__socials">
      {socials()
        .into_iter()
        .map(|e| view! { <SocialIcon default=e.default hover=e.hover /> })
        .collect_view()}
    </div>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Page Nav                                                  ║
//╚═══════════════════════════════════════════════════════════╝

/// All site pages as footer navigation links.
/// The footer shows every page — no filtering by current route.
#[component]
pub fn PageNav() -> impl IntoView {
  view! {
    <nav class="footer__pages">
      {PAGES
        .iter()
        .enumerate()
        .map(|(i, page)| {
          let divided = i < PAGES.len() - 1;
          view! {
            <a
              href=page.path
              title=page.description
              class=if divided { "footer__page footer__page--divided" } else { "footer__page" }
            >
              {page.label}
            </a>
          }
        })
        .collect_view()}
    </nav>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Copyright                                                 ║
//╚═══════════════════════════════════════════════════════════╝

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

//╔═══════════════════════════════════════════════════════════╗
//║ Root                                                      ║
//╚═══════════════════════════════════════════════════════════╝

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="footer">
      <Divider />
      <div class="footer__inner">
        <Socials />
        <PageNav />
      </div>
      <Divider config=Divider::default_with_dot() />
      <div class="footer__bottom">
        <Copyright />
      </div>
    </footer>
  }
}
