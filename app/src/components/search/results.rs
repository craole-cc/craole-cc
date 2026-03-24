use super::_prelude::*;

#[component]
#[allow(clippy::must_use_candidate)]
pub fn Results(
  query : ReadSignal<String,>,
  results : Resource<Result<Vec<Item,>, ServerFnError,>,>,
  set_open : WriteSignal<bool,>,
) -> impl IntoView {
  view! {
    <Suspense fallback=|| ()>
      {move || {
        results
          .get()
          .map(|res| {
            res
              .ok()
              .map(|items| {
                if items.is_empty() && query.get().len() >= 2 {
                  return view! { <p class="spotlight__empty">"No results found."</p> }.into_any();
                }
                let (mut dev, mut art, mut log) = (vec![], vec![], vec![]);
                for item in items {
                  match item.kind {
                    Kind::Project => dev.push(item),
                    Kind::Art => art.push(item),
                    Kind::Log => log.push(item),
                  }
                }

                view! {
                  <div class="spotlight__results">
                    <ResultGroup
                      label="Dev"
                      class="spotlight__group-label--dev"
                      items=dev
                      set_open=set_open
                    />
                    <ResultGroup
                      label="Art"
                      class="spotlight__group-label--art"
                      items=art
                      set_open=set_open
                    />
                    <ResultGroup
                      label="Log"
                      class="spotlight__group-label--log"
                      items=log
                      set_open=set_open
                    />
                  </div>
                }
                  .into_any()
              })
          })
      }}
    </Suspense>
  }
}
