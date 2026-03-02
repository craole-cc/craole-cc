use crate::prelude::*;

#[component]
pub fn Admin() -> impl IntoView {
  view! {
    <section class="readable page page--admin">
      <header class="page__header">
        <h1 class="page__title">"Admin"</h1>
        <p class="page__sub">"Content management."</p>
      </header>

      <Divider />

      <p>"(Coming next) Create/edit posts, projects, media."</p>
    </section>
  }
}
