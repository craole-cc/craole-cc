/// Where an icon's visual asset comes from.
#[derive(Default, Clone)]
pub enum Source {
  #[default]
  Empty,
  Leptos(super::ico::Icon),
  Local(&'static str),
}
