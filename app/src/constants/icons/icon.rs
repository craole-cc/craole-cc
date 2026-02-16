use super::{
  IconData,
  Source,
};

/// A single resolved icon — source, optional Tailwind class, and optional metadata.
#[derive(Default, Clone)]
pub struct Icon {
  pub source: Source,
  pub class: &'static str,
  pub label: &'static str,
  pub tooltip: &'static str,
  pub link: &'static str,
}

impl Icon {
  pub fn leptos(icon: IconData) -> Self {
    Self {
      source: Source::Leptos(icon),
      ..Default::default()
    }
  }

  pub fn local(src: &'static str) -> Self {
    Self {
      source: Source::Local(src),
      ..Default::default()
    }
  }

  pub fn with_class(mut self, class: &'static str) -> Self {
    self.class = class;
    self
  }

  pub fn with_label(mut self, label: &'static str) -> Self {
    self.label = label;
    self
  }

  pub fn with_tooltip(mut self, tooltip: &'static str) -> Self {
    self.tooltip = tooltip;
    self
  }

  pub fn with_link(mut self, link: &'static str) -> Self {
    self.link = link;
    self
  }
}

impl From<IconData> for Icon {
  fn from(icon: IconData) -> Self {
    Icon::leptos(icon)
  }
}

impl From<&'static str> for Icon {
  fn from(src: &'static str) -> Self {
    Icon::local(src)
  }
}
