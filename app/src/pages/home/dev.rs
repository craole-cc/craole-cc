use crate::prelude::*;

#[component]
pub fn Stacks() -> impl IntoView {
  view! {
    <section id="tech-stack" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {}", NEUTRAL_TEXT_800)>"🧰 Tech Stack"</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {stacks().into_iter().map(|cat| view! { <StackCard category=cat /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn StackCard(category: Stack) -> impl IntoView {
  view! {
    <div class=format!("p-5 rounded-lg border {} {}", NEUTRAL_BG_100, NEUTRAL_BORDER_300)>
      <h3 class=format!("mb-4 text-lg font-semibold {}", NEUTRAL_TEXT_800)>{category.title}</h3>
      <div class="flex flex-wrap gap-3">
        {category
          .technologies
          .into_iter()
          .map(|tech| view! { <TechBadge tech=tech /> })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}

#[component]
fn TechBadge(tech: Tech) -> impl IntoView {
  view! {
    <a
      href=tech.link
      target="_blank"
      rel="noopener noreferrer"
      class=format!(
        "flex gap-2 items-center py-2 px-3 rounded-lg border \
        transition-all hover:shadow-md hover:scale-105 group \
        {} {} hover:{} dark:text-white",
        NEUTRAL_BG_CARD,
        NEUTRAL_BORDER_300,
        PRIMARY_BORDER_400,
      )
    >
      <span class="flex justify-center items-center w-5 h-5">
        <RenderIcon icon=tech.logo />
      </span>
      <span class=format!("text-sm font-medium {}", NEUTRAL_TEXT_700)>{tech.name}</span>
    </a>
  }
}

#[component]
pub fn Areas() -> impl IntoView {
  view! {
    <section id="what-i-build" class="mb-20">
      <h2 class=format!("mb-8 text-4xl font-bold {}", NEUTRAL_TEXT_800)>"🛠️ What I Build"</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        {areas().into_iter().map(|area| view! { <AreaCard area=area /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn AreaCard(area: Area) -> impl IntoView {
  view! {
    <div class=format!(
      "p-6 rounded-xl border transition-all duration-300 \
          hover:shadow-lg {} {}",
      NEUTRAL_BG_SURFACE,
      NEUTRAL_BORDER_300,
    )>
      <h3 class=format!("mb-4 text-2xl font-semibold {}", NEUTRAL_TEXT_800)>
        <span class="mr-2">{area.icon}</span>
        {area.title}
      </h3>
      <ul class="space-y-2">
        {area
          .points
          .into_iter()
          .map(|point| {
            view! {
              <li class=format!("flex items-start {}", NEUTRAL_TEXT_700)>
                <span class=format!("mr-2 {}", PRIMARY_TEXT_600)>"•"</span>
                <span>{point}</span>
              </li>
            }
          })
          .collect::<Vec<_>>()}
      </ul>
    </div>
  }
}
