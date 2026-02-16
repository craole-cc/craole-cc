// Renders a hard-black SVG as mid-grey:
//   brightness(0)  — collapses all pixels to black
//   invert(0.35)   — lifts to ~35% grey (tweak to taste)
pub const GREY_FROM_BLACK: &str = "filter: brightness(0) invert(0.35);";

// Dims a solid-colour SVG (e.g. LinkedIn, Facebook) to match the grey tone
// of GREY_FROM_BLACK icons without flattening the image to a solid block.
pub const DIM_COLOUR: &str = "opacity: 0.6;";

//╔═══════════════════════════════════════════════════════════╗
//║ Icon                                                      ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub struct Icon {
  pub src: &'static str,
  pub class: Option<&'static str>,
  pub style: Option<&'static str>,
}

impl Icon {
  pub fn new(src: &'static str) -> Self {
    Self {
      src,
      ..Default::default()
    }
  }

  pub fn with_class(mut self, class: &'static str) -> Self {
    self.class = Some(class);
    self
  }

  pub fn with_style(mut self, style: &'static str) -> Self {
    self.style = Some(style);
    self
  }

  pub fn class(&self) -> &'static str {
    self.class.unwrap_or_default()
  }

  pub fn style(&self) -> &'static str {
    self.style.unwrap_or_default()
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Logo                                                      ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub struct Logo {
  pub reset: Icon,
  pub hover: Icon,
}

impl Logo {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn new_icon(reset: Icon) -> Self {
    let hover = reset.clone();
    Self { reset, hover }
  }

  pub fn new_icon_src(path: &'static str) -> Self {
    Self::new_icon(Icon::new(path))
  }

  pub fn with_hover(mut self, icon: Icon) -> Self {
    self.hover = icon;
    self
  }

  pub fn with_icon_src(mut self, path: &'static str) -> Self {
    self.reset.src = path;
    self.hover.src = path;
    self
  }

  pub fn with_icon_class(mut self, class: &'static str) -> Self {
    self.reset.class = Some(class);
    self
  }

  pub fn with_icon_style(mut self, style: &'static str) -> Self {
    self.reset.style = Some(style);
    self
  }

  pub fn with_icon_hover_src(mut self, path: &'static str) -> Self {
    self.hover.src = path;
    self
  }

  pub fn with_icon_hover_class(mut self, class: &'static str) -> Self {
    self.hover.class = Some(class);
    self
  }

  pub fn with_icon_hover_style(mut self, style: &'static str) -> Self {
    self.hover.style = Some(style);
    self
  }
}

//╔═══════════════════════════════════════════════════════════╗
//║ Socials                                                   ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
pub struct Social {
  pub link: &'static str,
  pub name: &'static str,
  pub logo: Logo,
}

pub fn socials() -> Vec<Social> {
  vec![
    Social {
      link: "mailto:craig.craole.cole@gmail.com",
      name: "Gmail",
      // logo: Logo::new_icon(Icon::new("icons/logos/gmail.svg")),
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
        .with_icon_style(Box::leak(
          format!("{GREY_FROM_BLACK} transform: scale(0.75);").into_boxed_str(),
        )),
    },
  ]
}
