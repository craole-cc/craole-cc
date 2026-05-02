use {
  super::config::{Divider, units_to_rem},
  crate::prelude::*,
};

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Divider(#[prop(default = Divider::default())] config: Divider) -> impl IntoView {
  let (left_gap, right_gap) = config.dot_pos.unwrap_or((0, 0));

  let style = format!(
    "--divider-padding: {}; --divider-gap-left: {}; --divider-gap-right: {};",
    config.padding,
    units_to_rem(left_gap),
    units_to_rem(right_gap),
  );

  let class = if config.show_dot {
    "divider divider--centered"
  } else {
    "divider"
  };

  view! {
    <div role="separator" aria-hidden="true" class=class style=style>
      <div class="divider__line divider__line--left" />
      <div class="divider__spacer divider__spacer--left" />
      {config.show_dot.then(|| view! { <span class="divider__dot" /> })}
      <div class="divider__spacer divider__spacer--right" />
      <div class="divider__line divider__line--right" />
    </div>
  }
}
