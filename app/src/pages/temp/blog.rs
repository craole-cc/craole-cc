use leptos::html::{Meta, Title};

use crate::prelude::*;

#[component]
pub fn BlogPost1() -> impl IntoView {
  let post_data = Resource::new_blocking(/* load blog post */);
  let comments_data = Resource::new(/* load blog comments */);
  view! {
    <Suspense fallback=|| ()>
      {move || Suspend::new(async move {
        let data = post_data.await;
        view! {
          <Title text=data.title />
          <Meta name="description" content=data.excerpt />
          <article></article>
        }
      })}
    </Suspense>
    <Suspense fallback=|| {
      "Loading comments..."
    }>
      {move || Suspend::new(async move {
        let comments = comments_data.await;
        todo!()
      })}
    </Suspense>
  }
}
