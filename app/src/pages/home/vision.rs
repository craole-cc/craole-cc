// #![allow(clippy::must_use_candidate)]
use crate::prelude::*;

#[component]
#[must_use]
pub fn Vision() -> impl IntoView {
  view! {
    <section class="vision">
      <div class="vision__inner">
        <div class="vision__label">
          <span>"The Vision"</span>
        </div>
        <div class="vision__statement">
          <p>
            "Raised on rhythm, I've always expressed myself through structure —
            first through music, now through code. The instrument changed.
            The intent never did."
          </p>
        </div>
      </div>
    </section>
  }
}
