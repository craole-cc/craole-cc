use leptos::prelude::*;

#[component]
pub fn contact() -> impl IntoView {
  view! {
    <section id="contact" class="mb-16">
      <h2 class="mb-8 text-4xl font-bold text-blue-400">"Get In Touch"</h2>
      <p class="mx-auto mb-10 max-w-2xl text-lg text-center text-slate-300">
        "Interested in backend development, data engineering, or collaboration opportunities? Let's connect!"
      </p>
      <div class="flex flex-wrap gap-4 justify-center">
        <a
          href="https://github.com/yourusername"
          class="py-4 px-8 font-semibold text-blue-400 rounded-lg border-2 border-blue-500 transition-all duration-300 hover:text-white hover:bg-blue-500 group bg-slate-800"
          target="_blank"
        >
          "GitHub"
        </a>
        <a
          href="https://linkedin.com/in/yourprofile"
          class="py-4 px-8 font-semibold text-blue-400 rounded-lg border-2 border-blue-500 transition-all duration-300 hover:text-white hover:bg-blue-500 group bg-slate-800"
          target="_blank"
        >
          "LinkedIn"
        </a>
        <a
          href="mailto:your.email@example.com"
          class="py-4 px-8 font-semibold text-blue-400 rounded-lg border-2 border-blue-500 transition-all duration-300 hover:text-white hover:bg-blue-500 group bg-slate-800"
        >
          "Email"
        </a>
      </div>
    </section>
  }
}
