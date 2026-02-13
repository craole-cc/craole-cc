// Renders a hard-black SVG as mid-grey:
//   brightness(0)  — collapses all pixels to black
//   invert(0.35)   — lifts to ~35% grey (tweak to taste)
pub const GREY_FROM_BLACK: &str = "filter: brightness(0) invert(0.35);";

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
  pub fn new(reset: Icon) -> Self {
    // Hover reuses the same src but starts with no class/style —
    // it should always appear at full colour unless explicitly overridden.
    let hover = Icon::new(reset.src);
    Self { reset, hover }
  }

  pub fn with_hover(mut self, icon: Icon) -> Self {
    self.hover = icon;
    self
  }
}
