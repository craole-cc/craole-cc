use crate::prelude::{
  icons::*,
  *,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Facebook                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod facebook {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/facebook.svg",)
      .with_link("https://facebook.com/craole",)
      .with_tooltip("Connect on Facebook",)
      .with_label("Facebook",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiFacebookBoxLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiFacebookBoxLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaFacebookFBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareFacebookBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_FACEBOOK_LIGHT, CLR_FACEBOOK_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gmail                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gmail {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/gmail.svg",)
      .with_link("mailto:craig.craole.cole@gmail.com",)
      .with_tooltip("Send me an email",)
      .with_label("Gmail",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::MdiGmail,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::TbBrandGmailOutline,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::BiGmail,),) }
  pub fn fa_square() -> Icon { fa_brands() }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_GMAIL_LIGHT, CLR_GMAIL_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Instagram                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod instagram {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/instagram.svg",)
      .with_link("https://instagram.com/craole",)
      .with_tooltip("Follow me on Instagram",)
      .with_label("Instagram",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiInstagramLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiInstagramLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaInstagramBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareInstagramBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_INSTAGRAM_ROSE, CLR_INSTAGRAM_BRIGHT_RED,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ LinkedIn                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linkedin {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/linkedin.svg",)
      .with_link("https://linkedin.com/in/craole",)
      .with_tooltip("Connect on LinkedIn",)
      .with_label("LinkedIn",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiLinkedinBoxLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiLinkedinBoxLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaLinkedinInBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaLinkedinBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().and_class(fill_class(CLR_LINKEDIN_LIGHT, CLR_LINKEDIN_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ WhatsApp                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod whatsapp {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/whatsapp.svg",)
      .with_link("https://wa.me/18768130049",)
      .with_tooltip("Message me on WhatsApp",)
      .with_label("WhatsApp",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { filled() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiWhatsappLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiWhatsappLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaWhatsappBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareWhatsappBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().with_class(fill_class(CLR_WHATSAPP_LIGHT, CLR_WHATSAPP_DARK,),)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ X (formerly Twitter)                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod x {
  use super::*;

  fn base() -> Icon {
    Icon::new_local("icons/logos/x.svg",)
      .with_link("https://x.com/craole",)
      .with_tooltip("Follow me on X",)
      .with_label("X",)
  }

  pub fn local() -> Icon { base() }
  pub fn default() -> Icon { local() }
  pub fn filled() -> Icon { base().with_source(Source::Leptos(icon::RiTwitterXLogosFill,),) }
  pub fn outlined() -> Icon { base().with_source(Source::Leptos(icon::RiTwitterXLogosLine,),) }
  pub fn fa_brands() -> Icon { base().with_source(Source::Leptos(icon::FaXTwitterBrands,),) }
  pub fn fa_square() -> Icon { base().with_source(Source::Leptos(icon::FaSquareXTwitterBrands,),) }

  pub fn with_color(variant : fn() -> Icon,) -> Icon {
    variant().with_class(fill_class(CLR_X_LIGHT, CLR_X_DARK,),)
  }
}
