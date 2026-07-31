use crate::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
  view! {
    <div class="contact-page">
      <Contact />
    </div>
  }
}

pub mod prelude {
  pub use super::ContactPage;
}
