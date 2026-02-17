use super::IconData;

/// Where an icon's visual asset comes from.
#[derive(Default, Clone, Copy)]
pub enum Source {
  #[default]
  Empty,
  Leptos(IconData),
  Local(&'static str),
}

/// A single icon with all its metadata.
#[derive(Default, Clone, Copy)]
pub struct Icon {
  pub source: Source,
  pub class: &'static str,
  pub label: &'static str,
  pub tooltip: &'static str,
  pub link: &'static str,
}

impl Icon {
  pub const fn new() -> Self {
    Self {
      source: Source::Empty,
      class: "",
      label: "",
      tooltip: "",
      link: "",
    }
  }

  pub const fn new_leptos(src: IconData) -> Self {
    Self::new().via_leptos(src)
  }

  pub const fn via_leptos(mut self, src: IconData) -> Self {
    self.source = Source::Leptos(src);
    self
  }

  pub const fn new_local(src: &'static str) -> Self {
    Self::new().via_local(src)
  }

  pub const fn via_local(mut self, src: &'static str) -> Self {
    self.source = Source::Local(src);
    self
  }

  /// Replace just the source while keeping other metadata
  pub const fn with_source(mut self, source: Source) -> Self {
    self.source = source;
    self
  }

  /// Append additional classes to existing ones
  pub fn and_class(mut self, additional: &'static str) -> Self {
    if self.class.is_empty() {
      self.class = additional;
    } else {
      self.class = Box::leak(format!("{} {}", self.class, additional).into_boxed_str());
    }
    self
  }

  /// Clear the class while keeping other metadata
  pub const fn without_class(mut self) -> Self {
    self.class = "";
    self
  }

  pub const fn with_class(mut self, class: &'static str) -> Self {
    self.class = class;
    self
  }

  pub const fn with_label(mut self, label: &'static str) -> Self {
    self.label = label;
    self
  }

  pub const fn with_tooltip(mut self, tooltip: &'static str) -> Self {
    self.tooltip = tooltip;
    self
  }

  pub const fn with_link(mut self, link: &'static str) -> Self {
    self.link = link;
    self
  }

  // Accessor methods
  pub const fn source(&self) -> &Source {
    &self.source
  }

  pub const fn class(&self) -> &'static str {
    self.class
  }

  pub const fn label(&self) -> &'static str {
    self.label
  }

  pub const fn tooltip(&self) -> &'static str {
    self.tooltip
  }

  pub const fn link(&self) -> &'static str {
    self.link
  }
}

impl From<IconData> for Icon {
  fn from(src: IconData) -> Self {
    Icon::new_leptos(src)
  }
}

impl From<&'static str> for Icon {
  fn from(src: &'static str) -> Self {
    Icon::new_local(src)
  }
}
