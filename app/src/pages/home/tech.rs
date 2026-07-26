use crate::prelude::*;

#[component]
pub fn Stacks() -> impl IntoView {
  view! {
    <section id="tech-stack" class="stacks" aria-labelledby="tech-stack-heading">
      <h2 id="tech-stack-heading">"🧰 Tech Stack"</h2>
      // Treat this as a semantic list of cards
      <ul class="stacks__grid">
        {STACKS.iter().map(|s| view! { <StackCard stack=s /> }).collect_view()}
      </ul>
    </section>
  }
}

#[component]
fn StackCard(stack : &'static Stack,) -> impl IntoView {
  view! {
    <li>
      <article class="stack-card">
        <h3>{stack.title}</h3>
        <ul>
          {stack
            .icons
            .iter()
            .map(|icon| {
              let icon = icon();
              view! { <TechBadge icon /> }
            })
            .collect_view()}
        </ul>
      </article>
    </li>
  }
}

/// A single technology badge — link + icon + label, all sourced from the Icon.
#[component]
fn TechBadge(icon : Icon,) -> impl IntoView {
  view! {
    <li>
      <a
        href=icon.link()
        target="_blank"
        rel="noopener noreferrer"
        aria-label=icon.label()
        title=icon.tooltip()
      >
        <IconRender icon />
        <span>{icon.label()}</span>
      </a>
    </li>
  }
}
