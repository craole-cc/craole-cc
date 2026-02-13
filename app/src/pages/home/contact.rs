use {
  crate::prelude::*,
  leptos::prelude::*,
};

//╔═══════════════════════════════════════════════════════════╗
//║ Data                                                      ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Default, Clone)]
struct Social {
  link: &'static str,
  name: &'static str,
  logo: Logo,
}

fn socials() -> Vec<Social> {
  vec![
    Social {
      link: "mailto:craig.craole.cole@gmail.com",
      name: "Gmail",
      //? Coloured SVG — default grayscale+opacity works fine
      logo: Logo::new(Icon::new("icons/logos/gmail.svg")),
    },
    Social {
      link: "https://github.com/craole-cc",
      name: "GitHub",
      //? Hard black SVG — use filter to render as grey
      logo: Logo::new(Icon::new("icons/logos/github.svg").with_style(GREY_FROM_BLACK)),
    },
    Social {
      link: "https://linkedin.com/in/craole",
      name: "LinkedIn",
      logo: Logo::new(Icon::new("icons/logos/linkedin.svg")),
    },
    Social {
      link: "https://wa.me/18768130049",
      name: "WhatsApp",
      //? Hard black SVG — use filter to render as grey
      logo: Logo::new(Icon::new("icons/logos/whatsapp-simple.svg").with_style(GREY_FROM_BLACK))
        .with_hover(Icon::new("icons/logos/whatsapp.svg")),
    },
    Social {
      link: "https://instagram.com/craole",
      name: "Instagram",
      //? Coloured SVG — default grayscale+opacity works fine
      logo: Logo::new(Icon::new("icons/logos/instagram-simple.svg"))
        .with_hover(Icon::new("icons/logos/instagram.svg")),
    },
    Social {
      link: "https://facebook.com/craole",
      name: "Facebook",
      //? Coloured SVG, no hover variant — same icon goes full colour on hover
      logo: Logo::new(Icon::new("icons/logos/facebook.svg")),
    },
    Social {
      link: "https://x.com/craole",
      name: "X",
      //? Hard black SVG — use filter to render as grey
      logo: Logo::new(Icon::new("icons/logos/x-simple.svg").with_style(GREY_FROM_BLACK))
        .with_hover(Icon::new("icons/logos/x.svg")),
    },
  ]
}

//╔═══════════════════════════════════════════════════════════╗
//║ Components                                                ║
//╚═══════════════════════════════════════════════════════════╝
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
      <ul class="mx-auto mb-6 space-y-2 max-w-2xl text-center text-slate-600 dark:text-slate-400">
        <li>"🌐 Full-stack applications with modern Rust frameworks"</li>
        <li>"📊 Data engineering pipelines and infrastructure"</li>
        <li>"⚙️ Developer tools and systems utilities"</li>
        <li>"🎨 Creative technical experiments"</li>
      </ul>
      <Socials />
    </section>
  }
}

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
            "{class} opacity-100 grayscale group-hover:opacity-0 {}",
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
