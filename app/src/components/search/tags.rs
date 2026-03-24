use crate::prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Tags(
  #[allow(clippy::needless_pass_by_value)] tags : Vec<String,>,
  #[allow(clippy::needless_pass_by_value)] label : String,
  #[allow(clippy::needless_pass_by_value)] page : String,
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  view! {
    <div class="spotlight__tags">
      <span class="spotlight__tags-label">{label}</span>
      {tags
        .into_iter()
        .map(|tag| {
          let href = format!("{page}?tag={tag}");
          view! {
            <a
              class="spotlight__chip"
              href=href
              on:click=move |_| set_open.set(false)
            >
              {tag}
            </a>
          }
        })
        .collect_view()}
    </div>
  }
}
