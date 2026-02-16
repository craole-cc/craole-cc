/// Where an icon's visual asset comes from.
#[derive(Default, Clone)]
pub enum Source {
  #[default]
  Empty,
  Leptos(icondata::Icon),
  Local(&'static str),
}
