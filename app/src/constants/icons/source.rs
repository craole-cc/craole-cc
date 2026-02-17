/// Where an icon's visual asset comes from.
#[derive(Default, Clone, Copy)]
pub enum Source {
  #[default]
  Empty,
  Leptos(super::icon::Icon),
  Local(&'static str),
}
