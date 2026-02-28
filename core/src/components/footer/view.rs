use {
  super::{
    Copyright,
    PageNav,
    Socials,
  },
  crate::prelude::*,
};

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="footer">
      <Divider />
      <div class="footer__inner">
        <Socials />
        <PageNav />
      </div>
      <Divider config=Divider::default_with_dot() />
      <div class="footer__bottom">
        <Copyright />
      </div>
    </footer>
  }
}
