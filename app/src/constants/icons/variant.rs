use super::{
  Icon,
  IconData,
};

/// A light/dark icon pair.
///
/// The common case is both themes sharing the same source,
/// with the dark icon carrying extra Tailwind classes (invert, hue-rotate, etc.).
#[derive(Default, Clone)]
pub struct Variant {
  pub light: Icon,
  pub dark: Icon,
}

impl Variant {
  /// Both themes share the same Leptos icon.
  pub fn leptos(icon: IconData) -> Self {
    Self {
      light: Icon::leptos(icon),
      dark: Icon::leptos(icon),
    }
  }

  /// Both themes share the same local asset.
  pub fn local(src: &'static str) -> Self {
    Self {
      light: Icon::local(src),
      dark: Icon::local(src),
    }
  }

  /// Add Tailwind classes to only the dark icon — the common case.
  pub fn dark_class(mut self, class: &'static str) -> Self {
    self.dark.class = class;
    self
  }

  /// Add the same Tailwind classes to both icons.
  pub fn both_class(mut self, class: &'static str) -> Self {
    self.light.class = class;
    self.dark.class = class;
    self
  }

  /// Add label to both icons.
  pub fn with_label(mut self, label: &'static str) -> Self {
    self.light.label = label;
    self.dark.label = label;
    self
  }

  /// Add tooltip to both icons.
  pub fn with_tooltip(mut self, tooltip: &'static str) -> Self {
    self.light.tooltip = tooltip;
    self.dark.tooltip = tooltip;
    self
  }

  /// Add link to both icons.
  pub fn with_link(mut self, link: &'static str) -> Self {
    self.light.link = link;
    self.dark.link = link;
    self
  }

  /// Resolve to the correct icon given whether dark mode is active.
  pub fn resolve(&self, dark: bool) -> &Icon {
    if dark { &self.dark } else { &self.light }
  }
}
