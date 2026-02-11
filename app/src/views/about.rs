use leptos::prelude::*;

#[component]
pub fn about() -> impl IntoView {
    view! {
      <section id="about" class="mb-20">
        <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">"About Me"</h2>
        <div class="space-y-4 text-lg">
          <p class="leading-relaxed text-slate-700 dark:text-slate-300">
            "I'm a data-focused professional with 8+ years in Learning & Development and 3 years as a TEFL tutor. "
            "My passion lies in building backend data systems—designing pipelines, storage solutions, and interfaces "
            "that empower analysts and decision-makers."
          </p>
          <p class="leading-relaxed text-slate-700 dark:text-slate-300">
            "Currently exploring modern data tools like Delta Lake in Rust and Neo4j graph databases, "
            "while sharpening my SQL fundamentals. I bring a unique perspective from my background as a "
            "professional musician (bassist, singer, producer), which shapes my approach to backend challenges: "
            "precise, structured, and innovative."
          </p>
        </div>
      </section>
    }
}
