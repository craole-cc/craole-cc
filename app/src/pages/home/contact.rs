use leptos::prelude::*;

#[derive(Clone)]
struct Social {
  link: &'static str,
  logo: &'static str,
  name: &'static str,
}

#[component]
pub fn Socials() -> impl IntoView {
  let socials = vec![
    Social {
      link: "mailto:craig.craole.cole@gmail.com",
      logo: "icons/social/gmail.svg",
      name: "Gmail",
    },
    Social {
      link: "https://github.com/craole-cc",
      logo: "icons/tech/github.svg",
      name: "GitHub",
    },
    Social {
      link: "https://wa.me/18768130049",
      logo: "icons/social/whatsapp.svg",
      name: "WhatsApp",
    },
    Social {
      link: "https://instagram.com/craole",
      logo: "icons/social/instagram.svg",
      name: "Instagram",
    },
    Social {
      link: "https://facebook.com/craole",
      logo: "icons/social/fb.svg",
      name: "Facebook",
    },
    Social {
      link: "https://x.com/craole",
      logo: "icons/social/x.svg",
      name: "X",
    },
  ];

  view! {
    <section id="socials" class="flex flex-wrap gap-4 justify-center">
      {socials.into_iter().map(|social| view! { <Logo social=social /> }).collect::<Vec<_>>()}
    </section>
  }
}

#[component]
fn Logo(social: Social) -> impl IntoView {
  view! {
    <a
      href=social.link
      class="inline-flex justify-center items-center p-4 bg-white rounded-lg transition-all duration-300 hover:bg-blue-500 hover:shadow-lg hover:-translate-y-1 group dark:bg-slate-800"
      target="_blank"
      rel="noopener noreferrer"
      aria-label=social.name
    >
      <img
        src=social.logo
        alt=social.name
        class="w-6 h-6 transition-all duration-300 filter saturate-0 group-hover:filter-none"
      />
    </a>
  }
}

#[component]
pub fn Contact() -> impl IntoView {
  view! {
    <section id="contact">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">
        "📫 Let's Connect"
      </h2>
      <p class="mx-auto mb-6 max-w-2xl text-lg text-center text-slate-700 dark:text-slate-300">
        "Looking for " <strong>"rust-centric projects"</strong> " across the full spectrum:"
      </p>
      <ul class="mx-auto mb-10 space-y-2 max-w-2xl text-center text-slate-600 dark:text-slate-400">
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
      <Socials />
    </section>
  }
}
