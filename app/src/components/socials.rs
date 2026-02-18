//! # Socials Component
//!
//! Social media link data and the `<Socials />` render component.
//! `Social` is a view-model — it exists solely to describe how links render
//! in this component. To add, remove, or reorder links, edit [`socials()`] only.

use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Social                                                    ║
//╚═══════════════════════════════════════════════════════════╝
/// View-model for a single social media entry.
#[derive(Default, Clone,)]
pub struct Social {
  pub link : &'static str,
  pub name : &'static str,
  pub logo : Logo,
}

/// Returns the social links in display order.
///
/// Black SVGs (GitHub, WhatsApp, X) use [`GREY_FROM_BLACK`] at rest and
/// [`WHITE_FROM_BLACK`] on hover — grey → white progression in dark mode.
/// Colour SVGs (Gmail, LinkedIn, Instagram, Facebook) rely on the `grayscale`
/// Tailwind class applied by [`SocialIcon`] at rest, and reveal full colour on hover.
pub fn socials() -> Vec<Social,> {
  vec![
    Social {
      link : "mailto:craig.craole.cole@gmail.com",
      name : "Gmail",
      // Colour SVG — grayscale class handles dimming at rest
      logo : Logo::new().with_icon_src("icons/logos/gmail.svg",),
    },
    Social {
      link : "https://github.com/craole-cc",
      name : "GitHub",
      // Black SVG — rest: grey, hover: white
      logo : Logo::new()
        .with_icon_src("icons/logos/github.svg",)
        .with_icon_style(GREY_FROM_BLACK,)
        .with_icon_hover_style(WHITE_FROM_BLACK,),
    },
    Social {
      link : "https://linkedin.com/in/craole",
      name : "LinkedIn",
      // Colour SVG — grayscale class handles dimming at rest
      logo : Logo::new().with_icon_src("icons/logos/linkedin.svg",),
    },
    Social {
      link : "https://wa.me/18768130049",
      name : "WhatsApp",
      // Black SVG — needs brightness lift; hover shows full-colour version
      logo : Logo::new()
        .with_icon_src("icons/logos/whatsapp-simple.svg",)
        .with_icon_hover_src("icons/logos/whatsapp.svg",)
        .with_icon_style(GREY_FROM_BLACK,),
    },
    Social {
      link : "https://instagram.com/craole",
      name : "Instagram",
      // Colour SVG — grayscale class handles dimming at rest
      logo : Logo::new()
        .with_icon_src("icons/logos/instagram.svg",)
        .with_icon_hover_src("icons/logos/instagram.svg",),
    },
    Social {
      link : "https://facebook.com/craole",
      name : "Facebook",
      // Colour SVG — grayscale class handles dimming at rest
      logo : Logo::new().with_icon_src("icons/logos/facebook.svg",),
    },
    Social {
      link : "https://x.com/craole",
      name : "X",
      // Black SVG — rest: grey + scaled, hover: white + scaled
      logo : Logo::new()
        .with_icon_src("icons/logos/x-simple.svg",)
        .with_icon_hover_src("icons/logos/x.svg",)
        .with_icon_style(Box::leak(
          format!("{GREY_FROM_BLACK} transform: scale(0.75);").into_boxed_str(),
        ),)
        .with_icon_hover_style(Box::leak(
          format!("{WHITE_FROM_BLACK} transform: scale(0.75);").into_boxed_str(),
        ),),
    },
  ]
}

//╔═══════════════════════════════════════════════════════════╗
//║ Components                                                ║
//╚═══════════════════════════════════════════════════════════╝
/// Renders the row of social media icon links with hover animation.
///
/// Each icon displays a greyscale/dimmed state at rest and transitions to
/// the full-colour hover variant on interaction.
#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <section id="socials" class="flex flex-wrap gap-2 justify-center">
      {socials().into_iter().map(|social| view! { <SocialIcon social /> }).collect::<Vec<_>>()}
    </section>
  }
}

/// Single social media icon link with crossfade hover effect.
///
/// Reset state: greyscale + dimmed (via Tailwind `grayscale` + the icon's style).
/// Hover state: full colour, no filter.
#[component]
fn SocialIcon(social : Social,) -> impl IntoView {
  let base = "absolute inset-0 w-6 h-6 transition-opacity duration-300";

  view! {
    <a
      href=social.link
      target="_blank"
      rel="noopener noreferrer"
      aria-label=social.name
      class=format!(
        "inline-flex justify-center items-center p-3 {} rounded-lg \
              transition-all duration-300 hover:shadow-lg hover:-translate-y-1 group",
        NEUTRAL_BG_GHOST,
      )
    >
      <span class="block relative w-6 h-6">
        // Reset (idle) — greyscale filter ensures all icons look consistent at rest,
        // regardless of whether their style uses GREY_FROM_BLACK or DIM_COLOUR.
        <img
          // <-- use helper
          src=social.logo.reset.custom_src()
          alt=social.name
          class=format!(
            "{base} opacity-100 grayscale group-hover:opacity-0 {}",
            social.logo.reset.class(),
          )
          style=social.logo.reset.style()
        />
        // Hover — no filter classes, full colour restored
        <img
          // <-- use helper
          src=social.logo.hover.custom_src()
          alt=social.name
          class=format!("{base} opacity-0 group-hover:opacity-100 {}", social.logo.hover.class())
          style=social.logo.hover.style()
        />
      </span>
    </a>
  }
}
