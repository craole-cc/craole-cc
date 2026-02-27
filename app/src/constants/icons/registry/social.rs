use super::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Facebook                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod facebook {
  use super::*;

  /// Icon selector for [Facebook](https://facebook.com/craole).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](Facebook::get).
  /// For Font Awesome styles use [`FacebookExt`] with [`Extended`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiFacebookBoxLogosFill` with `--brand-facebook` |
  /// | `Outlined` | [`outlined`] — `RiFacebookBoxLogosLine` with `--brand-facebook` |
  ///
  /// # Example
  /// ```rust
  /// let icon = facebook::Facebook(Variant::Filled,).get(); 
  /// ```
  pub struct Facebook(pub Variant,);

  impl Facebook {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard Facebook icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaFacebookFBrands` with `--brand-facebook` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareFacebookBrands` with `--brand-facebook` |
  ///
  /// # Example
  /// ```rust
  /// let icon = facebook::FacebookExt(facebook::Extended::FaSquare,).get(); 
  /// ```
  pub struct FacebookExt(pub Extended,);

  impl FacebookExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome F-logo brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/facebook.svg",)
      .with_link("https://facebook.com/craole",)
      .with_tooltip("Connect on Facebook",)
      .with_label("Facebook",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiFacebookBoxLogosFill`](icon::RiFacebookBoxLogosFill) with `--brand-facebook`
  /// colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiFacebookBoxLogosFill,),)
      .colored("brand-facebook",)
  }
  /// Outlined [`RiFacebookBoxLogosLine`](icon::RiFacebookBoxLogosLine) with `--brand-facebook`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiFacebookBoxLogosLine,),)
      .colored("brand-facebook",)
  }
  /// Font Awesome F-logo [`FaFacebookFBrands`](icon::FaFacebookFBrands) with `--brand-facebook`
  /// colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaFacebookFBrands,),)
      .colored("brand-facebook",)
  }
  /// Font Awesome square [`FaSquareFacebookBrands`](icon::FaSquareFacebookBrands) with
  /// `--brand-facebook` colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareFacebookBrands,),)
      .colored("brand-facebook",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Gmail                                                     ║
//╚═══════════════════════════════════════════════════════════╝
pub mod gmail {
  use super::*;

  /// Icon selector for [Gmail](mailto:craig.craole.cole@gmail.com).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](Gmail::get).
  /// For the BoxIcons style use [`GmailExt`] with [`Extended`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `MdiGmail` with `--brand-gmail` |
  /// | `Outlined` | [`outlined`] — `TbBrandGmailOutline` with `--brand-gmail` |
  ///
  /// # Example
  /// ```rust
  /// let icon = gmail::Gmail(Variant::Filled,).get(); 
  /// ```
  pub struct Gmail(pub Variant,);

  impl Gmail {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard Gmail icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `BiGmail` BoxIcons with `--brand-gmail` |
  ///
  /// # Example
  /// ```rust
  /// let icon = gmail::GmailExt(gmail::Extended::FaBrands,).get(); 
  /// ```
  pub struct GmailExt(pub Extended,);

  impl GmailExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// BoxIcons brands icon — no square variant exists for Gmail.
    FaBrands,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/gmail.svg",)
      .with_link("mailto:craig.craole.cole@gmail.com",)
      .with_tooltip("Send me an email",)
      .with_label("Gmail",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }

  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }

  /// Filled [`MdiGmail`](icon::MdiGmail) with `--brand-gmail` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::MdiGmail,),)
      .colored("brand-gmail",)
  }
  /// Outlined [`TbBrandGmailOutline`](icon::TbBrandGmailOutline) with `--brand-gmail` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::TbBrandGmailOutline,),)
      .colored("brand-gmail",)
  }
  /// BoxIcons [`BiGmail`](icon::BiGmail) with `--brand-gmail` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::BiGmail,),)
      .colored("brand-gmail",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Instagram                                                 ║
//╚═══════════════════════════════════════════════════════════╝
pub mod instagram {
  use super::*;

  /// Icon selector for [Instagram](https://instagram.com/craole).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](Instagram::get).
  /// For Font Awesome styles use [`InstagramExt`] with [`Extended`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiInstagramLogosFill` with `--brand-instagram` |
  /// | `Outlined` | [`outlined`] — `RiInstagramLogosLine` with `--brand-instagram` |
  ///
  /// # Example
  /// ```rust
  /// let icon = instagram::Instagram(Variant::Outlined,).get(); 
  /// ```
  pub struct Instagram(pub Variant,);

  impl Instagram {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard Instagram icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaInstagramBrands` with `--brand-instagram` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareInstagramBrands` with `--brand-instagram` |
  ///
  /// # Example
  /// ```rust
  /// let icon = instagram::InstagramExt(instagram::Extended::FaBrands,).get(); 
  /// ```
  pub struct InstagramExt(pub Extended,);

  impl InstagramExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/instagram.svg",)
      .with_link("https://instagram.com/craole",)
      .with_tooltip("Follow me on Instagram",)
      .with_label("Instagram",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiInstagramLogosFill`](icon::RiInstagramLogosFill) with `--brand-instagram` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiInstagramLogosFill,),)
      .colored("brand-instagram",)
  }
  /// Outlined [`RiInstagramLogosLine`](icon::RiInstagramLogosLine) with `--brand-instagram` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiInstagramLogosLine,),)
      .colored("brand-instagram",)
  }
  /// Font Awesome [`FaInstagramBrands`](icon::FaInstagramBrands) with `--brand-instagram` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaInstagramBrands,),)
      .colored("brand-instagram",)
  }
  /// Font Awesome square [`FaSquareInstagramBrands`](icon::FaSquareInstagramBrands) with
  /// `--brand-instagram` colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareInstagramBrands,),)
      .colored("brand-instagram",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ LinkedIn                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod linkedin {
  use super::*;

  /// Icon selector for [LinkedIn](https://linkedin.com/in/craole).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](LinkedIn::get).
  /// For Font Awesome styles use [`LinkedInExt`] with [`Extended`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiLinkedinBoxLogosFill` with `--brand-linkedin` |
  /// | `Outlined` | [`outlined`] — `RiLinkedinBoxLogosLine` with `--brand-linkedin` |
  ///
  /// # Example
  /// ```rust
  /// let icon = linkedin::LinkedIn(Variant::Filled,).get(); 
  /// ```
  pub struct LinkedIn(pub Variant,);

  impl LinkedIn {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard LinkedIn icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaLinkedinInBrands` badge with `--brand-linkedin` |
  /// | `FaSquare` | [`fa_square`] — `FaLinkedinBrands` square with `--brand-linkedin` |
  ///
  /// # Example
  /// ```rust
  /// let icon = linkedin::LinkedInExt(linkedin::Extended::FaSquare,).get(); 
  /// ```
  pub struct LinkedInExt(pub Extended,);

  impl LinkedInExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome badge-style brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/linkedin.svg",)
      .with_link("https://linkedin.com/in/craole",)
      .with_tooltip("Connect on LinkedIn",)
      .with_label("LinkedIn",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiLinkedinBoxLogosFill`](icon::RiLinkedinBoxLogosFill) with `--brand-linkedin`
  /// colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiLinkedinBoxLogosFill,),)
      .colored("brand-linkedin",)
  }
  /// Outlined [`RiLinkedinBoxLogosLine`](icon::RiLinkedinBoxLogosLine) with `--brand-linkedin`
  /// colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiLinkedinBoxLogosLine,),)
      .colored("brand-linkedin",)
  }
  /// Font Awesome badge [`FaLinkedinInBrands`](icon::FaLinkedinInBrands) with `--brand-linkedin`
  /// colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaLinkedinInBrands,),)
      .colored("brand-linkedin",)
  }
  /// Font Awesome square [`FaLinkedinBrands`](icon::FaLinkedinBrands) with `--brand-linkedin`
  /// colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaLinkedinBrands,),)
      .colored("brand-linkedin",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ WhatsApp                                                  ║
//╚═══════════════════════════════════════════════════════════╝
pub mod whatsapp {
  use super::*;

  /// Icon selector for [WhatsApp](https://wa.me/18768130049).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](WhatsApp::get).
  /// For Font Awesome styles use [`WhatsAppExt`] with [`Extended`].
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`local`] — monochrome SVG |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiWhatsappLogosFill` with `--brand-whatsapp` |
  /// | `Outlined` | [`outlined`] — `RiWhatsappLogosLine` with `--brand-whatsapp` |
  ///
  /// # Example
  /// ```rust
  /// let icon = whatsapp::WhatsApp(Variant::Filled,).get(); 
  /// ```
  pub struct WhatsApp(pub Variant,);

  impl WhatsApp {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => local(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard WhatsApp icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaWhatsappBrands` with `--brand-whatsapp` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareWhatsappBrands` with `--brand-whatsapp` |
  ///
  /// # Example
  /// ```rust
  /// let icon = whatsapp::WhatsAppExt(whatsapp::Extended::FaSquare,).get(); 
  /// ```
  pub struct WhatsAppExt(pub Extended,);

  impl WhatsAppExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome brands icon.
    FaBrands,
    /// Font Awesome square brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/whatsapp.svg",)
      .with_link("https://wa.me/18768130049",)
      .with_tooltip("Message me on WhatsApp",)
      .with_label("WhatsApp",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`local`].
  pub fn default() -> Icon { local() }
  /// Filled [`RiWhatsappLogosFill`](icon::RiWhatsappLogosFill) with `--brand-whatsapp` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiWhatsappLogosFill,),)
      .colored("brand-whatsapp",)
  }
  /// Outlined [`RiWhatsappLogosLine`](icon::RiWhatsappLogosLine) with `--brand-whatsapp` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiWhatsappLogosLine,),)
      .colored("brand-whatsapp",)
  }
  /// Font Awesome [`FaWhatsappBrands`](icon::FaWhatsappBrands) with `--brand-whatsapp` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaWhatsappBrands,),)
      .colored("brand-whatsapp",)
  }
  /// Font Awesome square [`FaSquareWhatsappBrands`](icon::FaSquareWhatsappBrands) with
  /// `--brand-whatsapp` colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareWhatsappBrands,),)
      .colored("brand-whatsapp",)
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ X (formerly Twitter)                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub mod x {
  use super::*;

  /// Icon selector for [X](https://x.com/craole) (formerly Twitter).
  ///
  /// Wraps a [`Variant`] and resolves it via [`get`](X::get).
  /// For Font Awesome styles use [`XExt`] with [`Extended`].
  ///
  /// > **Note:** `Default` resolves to [`filled`] — the coloured Leptos icon
  /// > is preferred over the monochrome local asset for X.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `Default` | [`filled`] — `RiTwitterXLogosFill` with `--brand-x` |
  /// | `Local` | [`local`] — monochrome SVG |
  /// | `Filled` | [`filled`] — `RiTwitterXLogosFill` with `--brand-x` |
  /// | `Outlined` | [`outlined`] — `RiTwitterXLogosLine` with `--brand-x` |
  ///
  /// # Example
  /// ```rust
  /// let icon = x::X(Variant::Default,).get(); // resolves to filled
  /// ```
  pub struct X(pub Variant,);

  impl X {
    /// Resolves the wrapped [`Variant`] to an [`Icon`].
    /// `Default` resolves to [`filled`] for this brand.
    pub fn get(self,) -> Icon {
      match self.0 {
        | Variant::Default => filled(),
        | Variant::Local => local(),
        | Variant::Filled => filled(),
        | Variant::Outlined => outlined(),
      }
    }
  }

  /// Extended variant selector for non-standard X icon styles.
  ///
  /// # Variants
  /// | Variant | Resolves to |
  /// |---|---|
  /// | `FaBrands` | [`fa_brands`] — `FaXTwitterBrands` with `--brand-x` |
  /// | `FaSquare` | [`fa_square`] — `FaSquareXTwitterBrands` with `--brand-x` |
  ///
  /// # Example
  /// ```rust
  /// let icon = x::XExt(x::Extended::FaBrands,).get(); 
  /// ```
  pub struct XExt(pub Extended,);

  impl XExt {
    /// Resolves the wrapped [`Extended`] variant to an [`Icon`].
    pub fn get(self,) -> Icon {
      match self.0 {
        | Extended::FaBrands => fa_brands(),
        | Extended::FaSquare => fa_square(),
      }
    }
  }

  /// Non-standard visual styles beyond the shared [`Variant`] set.
  #[derive(Clone, Copy, PartialEq, Eq, Hash,)]
  pub enum Extended {
    /// Font Awesome X brands icon.
    FaBrands,
    /// Font Awesome square X brands icon.
    FaSquare,
  }

  fn base() -> Icon {
    Icon::new_local("icons/logos/x.svg",)
      .with_link("https://x.com/craole",)
      .with_tooltip("Follow me on X",)
      .with_label("X",)
  }

  /// Local SVG asset — monochrome, inherits colour from context.
  pub fn local() -> Icon { base() }
  /// Canonical default — resolves to [`filled`].
  pub fn default() -> Icon { filled() }
  /// Filled [`RiTwitterXLogosFill`](icon::RiTwitterXLogosFill) with `--brand-x` colour.
  pub fn filled() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiTwitterXLogosFill,),)
      .colored("brand-x",)
  }
  /// Outlined [`RiTwitterXLogosLine`](icon::RiTwitterXLogosLine) with `--brand-x` colour.
  pub fn outlined() -> Icon {
    base()
      .with_source(Source::Leptos(icon::RiTwitterXLogosLine,),)
      .colored("brand-x",)
  }
  /// Font Awesome [`FaXTwitterBrands`](icon::FaXTwitterBrands) with `--brand-x` colour.
  pub fn fa_brands() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaXTwitterBrands,),)
      .colored("brand-x",)
  }
  /// Font Awesome square [`FaSquareXTwitterBrands`](icon::FaSquareXTwitterBrands) with `--brand-x`
  /// colour.
  pub fn fa_square() -> Icon {
    base()
      .with_source(Source::Leptos(icon::FaSquareXTwitterBrands,),)
      .colored("brand-x",)
  }
}
