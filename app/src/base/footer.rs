use crate::_prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Social Icons                                              ║
//╚═══════════════════════════════════════════════════════════╝

/// Social platform entries rendered in the footer.
/// Each entry pairs a filled (hover) icon with a default (rest) icon.
struct SocialEntry {
  /// Coloured Leptos icon — shown on hover.
  filled :  Icon,
  /// Monochrome local SVG — shown at rest.
  default : Icon,
}

impl SocialEntry {
  fn new(default : Icon, filled : Icon,) -> Self { Self { default, filled, } }
}

fn socials() -> Vec<SocialEntry,> {
  vec![
    SocialEntry::new(gmail::local(), gmail::filled(),),
    SocialEntry::new(github::local(), github::filled(),),
    SocialEntry::new(linkedin::local(), linkedin::filled(),),
    SocialEntry::new(whatsapp::local(), whatsapp::filled(),),
    SocialEntry::new(instagram::local(), instagram::filled(),),
    SocialEntry::new(facebook::local(), facebook::filled(),),
    SocialEntry::new(x::local(), x::filled(),),
  ]
}

/// Renders a single social icon link with a monochrome rest state
/// and a coloured hover state.
#[component]
fn SocialIcon(
  /// Monochrome local SVG — shown at rest.
  default : Icon,
  /// Coloured Leptos icon — shown on hover.
  filled : Icon,
) -> impl IntoView {
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
        <IconRender icon=filled class="footer__social-svg" />
      </span>
    </a>
  }
}

/// Renders the full row of social platform links.
#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <div class="footer__socials">
      {socials()
        .into_iter()
        .map(|entry| view! {
          <SocialIcon
            default=entry.default
            filled=entry.filled
          />
        })
        .collect::<Vec<_>>()}
    </div>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Facet Nav                                                 ║
//╚═══════════════════════════════════════════════════════════╝

/// Renders the horizontal facet navigation links in the footer.
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
              class=if is_last {
                "footer__facet"
              } else {
                "footer__facet footer__facet--divided"
              }
            >
              {facet.label}
            </a>
          }
        })
        .collect::<Vec<_>>()}
    </nav>
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Copyright                                                 ║
//╚═══════════════════════════════════════════════════════════╝

/// Renders the author name, tech credits, and copyright notice.
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
          </a>
          " & "
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

/// Site-wide footer containing social links, facet navigation,
/// and copyright information.
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
      </div>
    </footer>
  }
}
