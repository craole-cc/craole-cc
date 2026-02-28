use crate::prelude::*;

// ── Tech Stack ────────────────────────────────────────────────────────────────

#[component]
pub fn Stacks() -> impl IntoView {
  view! {
    <section id="tech-stack" class="stacks">
      <h2>"🧰 Tech Stack"</h2>
      <div class="stacks__grid">
        {STACKS.iter().map(|s| view! { <StackCard stack=s /> }).collect_view()}
      </div>
    </section>
  }
}

#[component]
fn StackCard(stack : &'static Stack,) -> impl IntoView {
  view! {
    <article class="stack-card">
      <h3>{stack.title}</h3>
      <ul role="list">
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

// ── What I Build ──────────────────────────────────────────────────────────────

#[component]
pub fn Areas() -> impl IntoView {
  view! {
    <section id="what-i-build" class="areas">
      <h2>"🛠️ What I Build"</h2>
      <div class="areas__grid">
        {AREAS.iter().map(|a| view! { <AreaCard area=a /> }).collect_view()}
      </div>
    </section>
  }
}

#[component]
fn AreaCard(area : &'static Area,) -> impl IntoView {
  view! {
    <article class="area-card">
      <h3>
        <span aria-hidden="true">{area.emoji}</span>
        {area.title}
      </h3>
      <ul>{area.points.iter().map(|p| view! { <li>{*p}</li> }).collect_view()}</ul>
    </article>
  }
}
