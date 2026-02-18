use crate::prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Facebook                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub fn facebook() -> Icon {
  Icon::new_local("icons/logos/facebook.svg",)
    .with_link("https://facebook.com/craole",)
    .with_tooltip("Connect on Facebook",)
    .with_label("Facebook",)
}

pub mod facebook_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/facebook.svg",) }

  pub fn filled() -> Source { Source::Leptos(icon::RiFacebookBoxLogosFill,) }

  pub fn outlined() -> Source { Source::Leptos(icon::RiFacebookBoxLogosLine,) }

  pub fn fa_brands() -> Source { Source::Leptos(icon::FaFacebookFBrands,) }

  pub fn fa_square() -> Source { Source::Leptos(icon::FaSquareFacebookBrands,) }

  pub fn with_color(source : Source,) -> Icon {
    facebook()
      .with_source(source,)
      .and_class(fill_class(CLR_FACEBOOK_LIGHT, CLR_FACEBOOK_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gmail                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub fn gmail() -> Icon {
  Icon::new_local("icons/logos/gmail.svg",)
    .with_label("Gmail",)
    .with_link("mailto:craig.craole.cole@gmail.com",)
    .with_tooltip("Send me an email",)
}

pub mod gmail_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/gmail.svg",) }
  pub fn filled() -> Source { Source::Leptos(icon::MdiGmail,) }
  pub fn outlined() -> Source { Source::Leptos(icon::TbBrandGmailOutline,) }
  pub fn fa_brands() -> Source { Source::Leptos(icon::BiGmail,) }
  pub fn fa_square() -> Source { Source::Leptos(icon::BiGmail,) }

  pub fn with_color(source : Source,) -> Icon {
    gmail()
      .with_source(source,)
      .and_class(fill_class(CLR_GMAIL_LIGHT, CLR_GMAIL_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Instagram                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub fn instagram() -> Icon {
  Icon::new_local("icons/logos/instagram.svg",)
    .with_label("Instagram",)
    .with_link("https://instagram.com/craole",)
    .with_tooltip("Follow me on Instagram",)
}

pub mod instagram_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/instagram.svg",) }
  pub fn filled() -> Source { Source::Leptos(icon::RiInstagramLogosFill,) }
  pub fn outlined() -> Source { Source::Leptos(icon::RiInstagramLogosLine,) }
  pub fn fa_brands() -> Source { Source::Leptos(icon::FaInstagramBrands,) }
  pub fn fa_square() -> Source { Source::Leptos(icon::FaSquareInstagramBrands,) }

  pub fn with_color(source : Source,) -> Icon {
    instagram()
      .with_source(source,)
      .and_class(fill_class(CLR_INSTAGRAM_ROSE, CLR_INSTAGRAM_BRIGHT_RED,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ LinkedIn                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub fn linkedin() -> Icon {
  Icon::new_local("icons/logos/linkedin.svg",)
    .with_label("LinkedIn",)
    .with_link("https://linkedin.com/in/craole",)
    .with_tooltip("Connect on LinkedIn",)
}

pub mod linkedin_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/linkedin.svg",) }
  pub fn filled() -> Source { Source::Leptos(icon::RiLinkedinBoxLogosFill,) }
  pub fn outlined() -> Source { Source::Leptos(icon::RiLinkedinBoxLogosLine,) }
  pub fn fa_brands() -> Source { Source::Leptos(icon::FaLinkedinInBrands,) }
  pub fn fa_square() -> Source { Source::Leptos(icon::FaLinkedinBrands,) }

  pub fn with_color(source : Source,) -> Icon {
    linkedin()
      .with_source(source,)
      .and_class(fill_class(CLR_LINKEDIN_LIGHT, CLR_LINKEDIN_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ WhatsApp                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub fn whatsapp() -> Icon {
  Icon::new_local("icons/logos/whatsapp.svg",)
    .with_link("https://wa.me/18768130049",)
    .with_tooltip("Message me on WhatsApp",)
    .with_label("WhatsApp",)
}

pub mod whatsapp_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/whatsapp.svg",) }

  pub fn filled() -> Source { Source::Leptos(icon::RiWhatsappLogosFill,) }

  pub fn outlined() -> Source { Source::Leptos(icon::RiWhatsappLogosLine,) }

  pub fn fa_brands() -> Source { Source::Leptos(icon::FaWhatsappBrands,) }

  pub fn fa_square() -> Source { Source::Leptos(icon::FaSquareWhatsappBrands,) }

  /// Get variant with brand color applied
  pub fn with_color(source : Source,) -> Icon {
    whatsapp()
      .with_source(source,)
      .with_class(fill_class(CLR_WHATSAPP_LIGHT, CLR_WHATSAPP_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ X (formerly Twitter)                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub fn x() -> Icon {
  Icon::new_local("icons/logos/x.svg",)
    .with_link("https://x.com/craole",)
    .with_tooltip("Follow me on X",)
    .with_label("X",)
}

pub mod x_variants {
  use super::*;

  pub fn local() -> Source { Source::Local("icons/logos/x.svg",) }

  pub fn filled() -> Source { Source::Leptos(icon::RiTwitterXLogosFill,) }

  pub fn outlined() -> Source { Source::Leptos(icon::RiTwitterXLogosLine,) }

  pub fn fa_brands() -> Source { Source::Leptos(icon::FaXTwitterBrands,) }

  pub fn fa_square() -> Source { Source::Leptos(icon::FaSquareXTwitterBrands,) }

  pub fn with_color(source : Source,) -> Icon {
    x()
      .with_source(source,)
      .with_class(fill_class(CLR_X_LIGHT, CLR_X_DARK,),)
  }
}
