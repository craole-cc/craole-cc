use crate::prelude::*;

#[component]
pub fn Socials() -> impl IntoView {
  view! {
    <section id="socials" class="flex flex-wrap gap-2 justify-center">
      {socials().into_iter().map(|social| view! { <SocialIcon social /> }).collect::<Vec<_>>()}
    </section>
  }
}

#[component]
fn SocialIcon(social: Social) -> impl IntoView {
  let class = "absolute inset-0 w-6 h-6 transition-opacity duration-300";

  view! {
    <a
      href=social.link
      target="_blank"
      rel="noopener noreferrer"
      aria-label=social.name
      class="inline-flex justify-center items-center p-3 bg-transparent rounded-lg transition-all duration-300 hover:shadow-lg hover:-translate-y-1 group dark:bg-slate-800"
    >

      <span class="block relative w-6 h-6">
        <img
          src=social.logo.reset.src
          alt=social.name
          class=format!(
            "{class} opacity-100 grayscale filter: brightness(0) invert(0.5) group-hover:opacity-0 {}",
            social.logo.reset.class(),
          )
          style=social.logo.reset.style()
        />
        <img
          src=social.logo.hover.src
          alt=social.name
          class=format!("{class} opacity-0 group-hover:opacity-100 {}", social.logo.hover.class())
          style=social.logo.hover.style()
        />
      </span>
    </a>
  }
}

#[component]
pub fn Facets() -> impl IntoView {
  view! {
    <div class="flex gap-4 justify-center items-center text-sm uppercase text-slate-400 dark:text-slate-500">
      {FACETS
        .iter()
        .enumerate()
        .map(|(i, facet)| {
          view! {
            <>
              <span
                class="transition-colors cursor-help dark:hover:text-slate-300 hover:text-slate-600"
                title=facet.description
              >
                {facet.label}
              </span>
              {(i < FACETS.len() - 1)
                .then(|| {
                  view! { <div class="w-px h-4 bg-slate-300 dark:bg-slate-700"></div> }
                })}
            </>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}

#[component]
pub fn Copyright() -> impl IntoView {
  view! {
    // ~@ Identity
    <p class="mb-2 tracking-tight text-md text-slate-700 dark:text-slate-300">
      <span>{AUTHOR_FIRSTNAME}</span>
      <span class="text-xl font-semibold">" "{AUTHOR_ALIAS}" "</span>
      <span>{AUTHOR_SURNAME}</span>
    </p>

    // ~@ Build technology
    <p class="text-xs">
      "Built with " <span class="font-medium text-orange-600 dark:text-orange-400">"Rust"</span>
      " & " <span class="font-medium text-purple-600 dark:text-purple-400">"Leptos"</span>
    </p>

    // ~@ Copyright
    <p class="text-xs">"© " {COPYRIGHT_YEAR} " — All rights reserved"</p>
  }
}

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="text-center text-slate-400 dark:text-slate-500">

      <Divider />
      <nav class="grid gap-4">
        <Socials />
        <Facets />
      </nav>

      <Divider config=Divider::default_with_dot() />
      <Copyright />

    </footer>
  }
}
