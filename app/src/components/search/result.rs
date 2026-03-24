use super::_prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn ResultGroup(
  label : &'static str,
  class : &'static str,
  #[allow(clippy::needless_pass_by_value)] items : Vec<Item,>,
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  if items.is_empty() {
    return None;
  }
  Some(view! {
    <div class="spotlight__group">
      <h3 class=format!("spotlight__group-label {class}")>{label}</h3>
      <ul class="spotlight__list" role="list">
        {items
          .into_iter()
          .map(|item| {
            view! {
              <li>
                <a
                  href=item.url
                  class="spotlight__result"
                  on:click=move |_| set_open.set(false)
                >
                  <span class="spotlight__result-title">{item.title}</span>
                  <span class="spotlight__result-sub">{item.subtitle}</span>
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </div>
  },)
}
