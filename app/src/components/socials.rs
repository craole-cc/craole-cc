//! # Socials Component
//!
//! Social media link data and the `<Socials />` render component.
//! Both live here because `Social` is a view-model — it exists solely
//! to describe how links render in this component, not as reusable site data.
//!
//! To add, remove, or reorder links, edit [`socials()`] only.

use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Social                                                    ║
//╚═══════════════════════════════════════════════════════════╝
/// View-model for a single social media entry.
#[derive(Default, Clone)]
pub struct Social {
  pub link: &'static str,
  pub name: &'static str,
  pub logo: Logo,
}

/// Returns the social links in display order.
///
/// Icons using [`GREY_FROM_BLACK`] ship as black SVGs (e.g. GitHub, X).
/// Icons using [`DIM_COLOUR`] are branded colour SVGs (e.g. LinkedIn, Gmail).
/// Hover variants restore full colour on interaction where provided.
pub fn socials() -> Vec<Social> {
  vec![
    Social {
      link: "mailto:craig.craole.cole@gmail.com",
      name: "Gmail",
      logo: Logo::new()
        .with_icon_src("icons/logos/gmail.svg")
        .with_icon_style(DIM_COLOUR),
    },
    Social {
      link: "https://github.com/craole-cc",
      name: "GitHub",
      logo: Logo::new()
        .with_icon_src("icons/logos/github.svg")
        .with_icon_style(GREY_FROM_BLACK),
    },
    Social {
      link: "https://linkedin.com/in/craole",
      name: "LinkedIn",
      logo: Logo::new()
        .with_icon_src("icons/logos/linkedin.svg")
        .with_icon_style(DIM_COLOUR),
    },
    Social {
      link: "https://wa.me/18768130049",
      name: "WhatsApp",
      logo: Logo::new()
        .with_icon_src("icons/logos/whatsapp-simple.svg")
        .with_icon_hover_src("icons/logos/whatsapp.svg")
        .with_icon_style(GREY_FROM_BLACK),
    },
    Social {
      link: "https://instagram.com/craole",
      name: "Instagram",
      logo: Logo::new()
        .with_icon_src("icons/logos/instagram.svg")
        .with_icon_hover_src("icons/logos/instagram.svg"),
    },
    Social {
      link: "https://facebook.com/craole",
      name: "Facebook",
      logo: Logo::new()
        .with_icon_src("icons/logos/facebook.svg")
        .with_icon_style(DIM_COLOUR),
    },
    Social {
      link: "https://x.com/craole",
      name: "X",
      logo: Logo::new()
        .with_icon_src("icons/logos/x-simple.svg")
        .with_icon_hover_src("icons/logos/x.svg")
        // Box::leak is unavoidable here — combining two &'static str values
        // at runtime requires allocating and leaking to produce a &'static str.
        .with_icon_style(Box::leak(
          format!("{GREY_FROM_BLACK} transform: scale(0.75);").into_boxed_str(),
        )),
    },
  ]
}

//╔═══════════════════════════════════════════════════════════╗
//║ Component                                                 ║
//╚═══════════════════════════════════════════════════════════╝
/// Renders the row of social media icon links.
#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <nav class="flex gap-4 items-center">
      {socials()
        .into_iter()
        .map(|s| {
          view! {
            <a href=s.link target="_blank" rel="noopener noreferrer" title=s.name>
              <img
                src=s.logo.reset.src
                alt=s.name
                class=s.logo.reset.class()
                style=s.logo.reset.style()
              />
            </a>
          }
        })
        .collect_view()}
    </nav>
  }
}
