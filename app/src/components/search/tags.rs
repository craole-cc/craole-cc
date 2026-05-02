use crate::prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::needless_pass_by_value)]
pub fn Tags(
  tags: Vec<String>,
  label: String,
  page: String,
  set_open: WriteSignal<bool>,
) -> impl IntoView {
  // Clone before view! consumes label via into_render
  let aria_label = label.clone();

  view! {
    <nav class="spotlight__tags" aria-label=aria_label>
      <span class="spotlight__tags-label">{label}</span>
      {tags
        .into_iter()
        .map(|tag| {
          let href = format!("{page}?tag={tag}");
          view! {
            <a class="spotlight__chip" href=href on:click=move |_| set_open.set(false)>
              {tag}
            </a>
          }
        })
        .collect_view()}
    </nav>
  }
}
