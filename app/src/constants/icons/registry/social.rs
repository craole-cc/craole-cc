use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Facebook                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod facebook {
  use super::{Icon, Variant, icon,};

  pub struct Facebook(pub Variant,);
  impl Facebook {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct FacebookExt(pub Extended,);
  impl FacebookExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, FaSquare, }

  pub const TAGS : &[&str] = &["Facebook", "facebook", "fb"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/facebook.svg",)
      .with_link("https://facebook.com/craole",)
      .with_tooltip("Connect on Facebook",)
      .with_label("Facebook",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::RiFacebookBoxLogosFill,).colored("brand-facebook",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::RiFacebookBoxLogosLine,).colored("brand-facebook",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::FaFacebookFBrands,).colored("brand-facebook",) }
  #[must_use] pub fn fa_square() -> Icon { base().via_leptos(icon::FaSquareFacebookBrands,).colored("brand-facebook",) }
  #[must_use] pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gmail                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gmail {
  use super::{Icon, Variant, icon,};

  pub struct Gmail(pub Variant,);
  impl Gmail {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct GmailExt(pub Extended,);
  impl GmailExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, }

  pub const TAGS : &[&str] = &["Gmail", "gmail", "email", "Email"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/gmail.svg",)
      .with_link("mailto:info@craole.cc",)
      .with_tooltip("Send me an email",)
      .with_label("Gmail",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::MdiGmail,).colored("brand-gmail",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::TbBrandGmailOutline,).colored("brand-gmail",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::BiGmail,).colored("brand-gmail",) }
  #[must_use] pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Instagram                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod instagram {
  use super::{Icon, Variant, icon,};

  pub struct Instagram(pub Variant,);
  impl Instagram {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct InstagramExt(pub Extended,);
  impl InstagramExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, FaSquare, }

  pub const TAGS : &[&str] = &["Instagram", "instagram", "ig"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/instagram.svg",)
      .with_link("https://instagram.com/craole",)
      .with_tooltip("Follow me on Instagram",)
      .with_label("Instagram",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::RiInstagramLogosFill,).colored("brand-instagram",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::RiInstagramLogosLine,).colored("brand-instagram",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::FaInstagramBrands,).colored("brand-instagram",) }
  #[must_use] pub fn fa_square() -> Icon { base().via_leptos(icon::FaSquareInstagramBrands,).colored("brand-instagram",) }
  #[must_use] pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ LinkedIn                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linkedin {
  use super::{Icon, Variant, icon,};

  pub struct LinkedIn(pub Variant,);
  impl LinkedIn {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct LinkedInExt(pub Extended,);
  impl LinkedInExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, FaSquare, }

  pub const TAGS : &[&str] = &["LinkedIn", "linkedin", "li"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/linkedin.svg",)
      .with_link("https://linkedin.com/in/craole",)
      .with_tooltip("Connect on LinkedIn",)
      .with_label("LinkedIn",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::RiLinkedinBoxLogosFill,).colored("brand-linkedin",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::RiLinkedinBoxLogosLine,).colored("brand-linkedin",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::FaLinkedinInBrands,).colored("brand-linkedin",) }
  #[must_use] pub fn fa_square() -> Icon { base().via_leptos(icon::FaLinkedinBrands,).colored("brand-linkedin",) }
  #[must_use] pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ WhatsApp                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod whatsapp {
  use super::{Icon, Variant, icon,};

  pub struct WhatsApp(pub Variant,);
  impl WhatsApp {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => default(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  pub struct WhatsAppExt(pub Extended,);
  impl WhatsAppExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, FaSquare, }

  pub const TAGS : &[&str] = &["WhatsApp", "whatsapp", "wa"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/whatsapp.svg",)
      .with_link("https://wa.me/18768130049",)
      .with_tooltip("Message me on WhatsApp",)
      .with_label("WhatsApp",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::RiWhatsappLogosFill,).colored("brand-whatsapp",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::RiWhatsappLogosLine,).colored("brand-whatsapp",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::FaWhatsappBrands,).colored("brand-whatsapp",) }
  #[must_use] pub fn fa_square() -> Icon { base().via_leptos(icon::FaSquareWhatsappBrands,).colored("brand-whatsapp",) }
  #[must_use] pub fn default() -> Icon { filled() }
}

//╔═══════════════════════════════════════════════════════════╗
//║ X (formerly Twitter)                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod x {
  use super::{Icon, Variant, icon,};

  pub struct X(pub Variant,);
  impl X {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Local => local(),
        | Variant::Outlined => outlined(),
        | Variant::Default | Variant::Filled => filled(),
      }
    }
  }

  pub struct XExt(pub Extended,);
  impl XExt {
    #[must_use]
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended { FaBrands, FaSquare, }

  pub const TAGS : &[&str] = &["X", "x", "Twitter", "twitter"];

  const fn base() -> Icon {
    Icon::new_local("icons/logos/x.svg",)
      .with_link("https://x.com/craole",)
      .with_tooltip("Follow me on X",)
      .with_label("X",)
  }

  #[must_use] pub const fn local() -> Icon { base() }
  #[must_use] pub fn filled() -> Icon { base().via_leptos(icon::RiTwitterXLogosFill,).colored("brand-x",) }
  #[must_use] pub fn outlined() -> Icon { base().via_leptos(icon::RiTwitterXLogosLine,).colored("brand-x",) }
  #[must_use] pub fn fa_brands() -> Icon { base().via_leptos(icon::FaXTwitterBrands,).colored("brand-x",) }
  #[must_use] pub fn fa_square() -> Icon { base().via_leptos(icon::FaSquareXTwitterBrands,).colored("brand-x",) }
  #[must_use] pub fn default() -> Icon { filled() }
}
