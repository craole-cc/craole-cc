use crate::prelude::{
  icons::*,
  *,
};

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
      hover :   github::default(),
    },
    SocialEntry {
      default : linkedin::filled(),
      hover :   linkedin::default(),
    },
    SocialEntry {
      default : whatsapp::filled(),
      hover :   whatsapp::default(),
    },
    SocialEntry {
      default : instagram::filled(),
      hover :   instagram::default(),
    },
    SocialEntry {
      default : facebook::filled(),
      hover :   facebook::default(),
    },
    SocialEntry {
      default : x::filled(),
      hover :   x::default(),
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
