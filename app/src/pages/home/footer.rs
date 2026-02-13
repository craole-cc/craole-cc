use {
  crate::prelude::*,
  leptos::prelude::*,
};

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="relative mt-2 border-t border-slate-200 dark:border-slate-700/50">

      // Subtle gradient fade at the top edge
      <div class="absolute inset-x-0 -top-px h-px from-transparent to-transparent bg-linear-to-r via-orange-400/40" />

      <div class="py-10 px-6 mx-auto max-w-4xl">

        // Identity line
        <p class="mb-4 text-lg font-semibold tracking-tight text-center text-slate-700 dark:text-slate-300">
          {AUTHOR_NAME} " "
          <span class="font-normal text-slate-400 dark:text-slate-500">
            "'" {AUTHOR_NICKNAME} "'"
          </span>
        </p>

        // Facets — what lives here
        <div class="flex gap-4 justify-center items-center mb-6 text-sm uppercase text-slate-400 dark:text-slate-500">
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

        // Divider
        <div class="flex gap-3 justify-center items-center mb-6">
          <div class="h-px grow bg-slate-200 dark:bg-slate-700/60" />
          <div class="w-1.5 h-1.5 rounded-full bg-purple-400/60" />
          <div class="h-px grow bg-slate-200 dark:bg-slate-700/60" />
        </div>

        // Build technology
        <p class="text-xs text-center text-slate-400 dark:text-slate-500">
          "Built with " <span class="font-medium text-orange-600 dark:text-orange-400">"Rust"</span>
          " & " <span class="font-medium text-purple-600 dark:text-purple-400">"Leptos"</span>
        </p>

        // Copyright
        <p class="text-xs text-center text-slate-400 dark:text-slate-500">
          "© " {COPYRIGHT_YEAR} " — All rights reserved"
        </p>

      </div>
    </footer>
  }
}
