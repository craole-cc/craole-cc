use super::Source;

/// A single resolved icon — source, optional Tailwind class, and optional metadata.
#[derive(Default, Clone)]
pub struct Icon {
  pub source:  Source,
  pub class:   &'static str,
  pub label:   &'static str,
  pub tooltip: &'static str,
  pub link:    &'static str,
}

impl Icon {
  pub fn leptos(icon: icondata::Icon) -> Self {
    Self { source: Source::Leptos(icon), ..Default::default() }
  }

  pub fn local(src: &'static str) -> Self {
    Self { source: Source::Local(src), ..Default::default() }
  }

  pub fn with_class(mut self, class: &'static str) -> Self {
    self.class = class;
    self
  }
}

impl From<ico::Icon> for Icon {
  fn from(icon: ico::Icon) -> Self { Icon::leptos(icon) }
}

impl From<&'static str> for Icon {
  fn from(src: &'static str) -> Self { Icon::local(src) }
}
