use crate::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact" class="contact-section" aria-labelledby="contact-title">
      <div class="contact-section__inner readable">
        <div class="contact-section__copy">
          <p class="contact-section__eyebrow">"Have a problem worth solving?"</p>
          <h2 id="contact-title">"Let's make something useful."</h2>
          <p class="contact-section__lead">
            "Tell me what you are building, what is getting in the way, or what you want to make clearer. I work across Rust software, data systems, infrastructure, and thoughtful technical products."
          </p>
          <ul class="contact-section__topics">
            <li>"Rust and full-stack product development"</li>
            <li>"Data, analytics, and operational workflows"</li>
            <li>"Reproducible infrastructure and automation"</li>
            <li>"Technical ideas that need shape and momentum"</li>
          </ul>
        </div>
        <div class="contact-section__action">
          <p>"The best first message is simple: what are you trying to do, and where are you stuck?"</p>
          <a
            class="contact-section__button"
            href="mailto:info@craole.cc?subject=Let's%20work%20together"
          >
            "Start a conversation "
            <span aria-hidden="true">"→"</span>
          </a>
          <p class="contact-section__note">"Email me at info@craole.cc"</p>
        </div>
      </div>
    </section>
  }
}
