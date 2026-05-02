#![allow(clippy::must_use_candidate)]
use {
  super::{Copyright, PageNav, Socials},
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
      <div class="footer__info">
        <Copyright />
      </div>
    </footer>
  }
}
