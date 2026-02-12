use leptos::prelude::*;
#[derive(Clone)]
struct Area {
  icon: &'static str,
  title: &'static str,
  points: Vec<&'static str>,
}

#[derive(Clone)]
struct Stack {
  title: &'static str,
  technologies: Vec<Tech>,
}

#[derive(Clone)]
struct Tech {
  name: &'static str,
  logo: &'static str,
  link: &'static str,
}

#[component]
pub fn Stacks() -> impl IntoView {
  let categories = vec![
    Stack {
      title: "Languages & Core",
      technologies: vec![
        Tech {
          name: "Rust",
          // logo: "https://cdn.simpleicons.org/rust/000000",
          logo: "images/logos/rust.svg",
          link: "https://www.rust-lang.org/",
        },
        Tech {
          name: "Shell",
          logo: "images/logos/shellscript.svg",
          link: "https://pubs.opengroup.org/onlinepubs/9799919799/",
        },
        // Tech {
        //   name: "Python",
        //   logo: "https://cdn.simpleicons.org/python/3776AB",
        //   link: "https://www.python.org/",
        // },
        // Tech {
        //   name: "Zig",
        //   logo: "https://cdn.simpleicons.org/zig/F7A41D",
        //   link: "https://ziglang.org/",
        // },
      ],
    },
    Stack {
      title: "Web",
      technologies: vec![
        Tech {
          name: "Axum",
          logo: "images/logos/tokio.png",
          // logo: "https://cdn.simpleicons.org/tokio/000000",
          link: "https://github.com/tokio-rs/axum",
        },
        Tech {
          name: "HTMX",
          // logo: "https://cdn.simpleicons.org/htmx/3366CC",
          logo: "images/logos/htmx.svg",
          link: "https://htmx.org/",
        },
        Tech {
          name: "Leptos",
          // logo: "https://avatars.githubusercontent.com/u/118319153?s=48&v=4",
          logo: "images/logos/leptos.png",
          link: "https://leptos.dev/",
        },
        Tech {
          name: "Tailwind",
          // logo: "https://tailwindcss.com/_next/static/media/tailwindcss-mark.96ee6a5a.svg",
          logo: "images/logos/tailwind.svg",
          link: "https://tailwindcss.com/",
        },
      ],
    },
    Stack {
      title: "Data Engineering",
      technologies: vec![
        // Tech {
        //   name: "Apache Spark",
        //   // logo: "https://cdn.simpleicons.org/apachespark/E25A1C",
        //   logo: "images/logos/spark.svg",
        //   link: "https://spark.apache.org/",
        // },
        // Tech {
        //   name: "Databricks",
        //   logo: "https://cdn.simpleicons.org/databricks/FF3621",
        //   link: "https://www.databricks.com/",
        // },
        Tech {
          name: "Neo4j",
          logo: "https://cdn.simpleicons.org/neo4j/008CC1",
          link: "https://neo4j.com/",
        },
        Tech {
          name: "PostgreSQL",
          logo: "https://cdn.simpleicons.org/postgresql/316192",
          link: "https://www.postgresql.org/",
        },
      ],
    },
    Stack {
      title: "DevOps & Systems",
      technologies: vec![
        Tech {
          name: "Git",
          logo: "https://cdn.simpleicons.org/git/F05032",
          link: "https://git-scm.com/",
        },
        Tech {
          name: "GitHub",
          // logo: "https://cdn.simpleicons.org/github/181717",
          logo: "images/logos/github.svg",
          link: "https://github.com/",
        },
        Tech {
          name: "NixOS",
          // logo: "https://nixos.wiki/images/thumb/2/20/Home-nixos-logo.png/207px-Home-nixos-logo.png",
          logo: "images/logos/nixos.png",
          link: "https://nixos.org/",
        },
        Tech {
          name: "Windows",
          // logo: "https://upload.wikimedia.org/wikipedia/commons/thumb/8/87/Windows_logo_-_2021.svg/960px-Windows_logo_-_2021.svg.png",
          logo: "images/logos/windows.png",
          link: "https://www.microsoft.com/windows",
        },
      ],
    },
    Stack {
      title: "Editors",
      technologies: vec![
        Tech {
          name: "Helix",
          logo: "https://helix-editor.com/logo.svg",
          link: "https://helix-editor.com/",
        },
        Tech {
          name: "Typst",
          // logo: "https://avatars.githubusercontent.com/u/67595261?s=48&v=4",
          logo: "https://cdn.simpleicons.org/typst/239DAD",
          link: "https://typst.app/",
        },
        Tech {
          name: "VS Code",
          logo: "https://code.visualstudio.com/assets/branding/code-stable.png",
          link: "https://code.visualstudio.com/",
        },
        Tech {
          name: "Zed",
          logo: "https://zed.dev/logo_icon.webp",
          link: "https://zed.dev/",
        },
      ],
    },
    Stack {
      title: "Terminal",
      technologies: vec![
        Tech {
          name: "Bash",
          logo: "https://cdn.simpleicons.org/gnubash/4EAA25",
          link: "https://www.gnu.org/software/bash/",
        },
        Tech {
          name: "PowerShell",
          logo: "https://raw.githubusercontent.com/gist/Xainey/d5bde7d01dcbac51ac951810e94313aa/raw/6c858c46726541b48ddaaebab29c41c07a196394/PowerShell.svg",
          link: "https://github.com/PowerShell/PowerShell",
        },
        Tech {
          name: "Starship",
          logo: "https://cdn.simpleicons.org/starship/DD0B78",
          link: "https://starship.rs/",
        },
        Tech {
          name: "Oh My Posh",
          logo: "https://ohmyposh.dev/img/logo-dark.svg",
          link: "https://ohmyposh.dev/",
        },
      ],
    },
  ];

  view! {
    <section id="tech-stack" class="mb-20">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">"🧰 Tech Stack"</h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {categories.into_iter().map(|cat| view! { <Stack category=cat /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn Stack(category: Stack) -> impl IntoView {
  view! {
    <div class="p-5 border rounded-lg bg-slate-50 border-slate-200 dark:bg-slate-800/30 dark:border-slate-700/50">
      <h3 class="mb-4 text-lg font-semibold text-slate-800 dark:text-slate-200">
        {category.title}
      </h3>
      <div class="flex flex-wrap gap-3">
        {category
          .technologies
          .into_iter()
          .map(|tech| view! { <Badge tech=tech /> })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}

#[component]
fn Badge(tech: Tech) -> impl IntoView {
  view! {
    <a
      href=tech.link
      target="_blank"
      rel="noopener noreferrer"
      class="flex items-center gap-2 px-3 py-2 transition-all bg-white border rounded-lg hover:border-blue-400 hover:shadow-md hover:scale-105 group border-slate-300 dark:bg-slate-800 dark:border-slate-600 dark:hover:border-blue-500"
    >
      <img
        src=tech.logo
        alt=tech.name
        class="object-contain w-5 h-5"
        loading="lazy"
        onerror="this.style.display='none'"
      />
      <span class="text-sm font-medium text-slate-700 dark:text-slate-300">{tech.name}</span>
    </a>
  }
}

#[component]
pub fn Areas() -> impl IntoView {
  let areas = vec![
    Area {
      icon: "🌐",
      title: "Full-Stack Development",
      points: vec![
        "Web applications with Rust backends (Axum, Actix) + modern frontends (HTMX, SPA frameworks)",
        "End-to-end solutions from database to UI",
      ],
    },
    Area {
      icon: "📊",
      title: "Data Engineering",
      points: vec![
        "High-performance data pipelines with Rust & Delta Lake",
        "Graph databases (Neo4j) for complex relationship modeling",
        "Analytics infrastructure that scales",
      ],
    },
    Area {
      icon: "⚙️",
      title: "Systems Utilities",
      points: vec![
        "Command-line tools that solve everyday problems elegantly",
        "Cross-platform applications (Windows, Linux, macOS)",
        "Developer productivity tools",
      ],
    },
    Area {
      icon: "🔧",
      title: "Infrastructure",
      points: vec![
        "Declarative NixOS configurations for reproducible environments",
        "Infrastructure-as-code and automation",
        "Self-hosted solutions",
      ],
    },
  ];

  view! {
    <section id="what-i-build" class="mb-20">
      <h2 class="mb-8 text-4xl font-bold dark:text-blue-400 text-slate-900">
        "🛠️ What I Build"
      </h2>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
        {areas.into_iter().map(|area| view! { <Area area=area /> }).collect::<Vec<_>>()}
      </div>
    </section>
  }
}

#[component]
fn Area(area: Area) -> impl IntoView {
  view! {
    <div class="p-6 transition-all duration-300 border rounded-xl hover:shadow-lg bg-white/80 border-slate-200 dark:bg-slate-800/50 dark:border-slate-700/50">
      <h3 class="mb-4 text-2xl font-semibold text-slate-800 dark:text-slate-100">
        <span class="mr-2">{area.icon}</span>
        {area.title}
      </h3>
      <ul class="space-y-2">
        {area
          .points
          .into_iter()
          .map(|point| {
            view! {
              <li class="flex items-start text-slate-700 dark:text-slate-300">
                <span class="mr-2 text-blue-500 dark:text-blue-400">"•"</span>
                <span>{point}</span>
              </li>
            }
          })
          .collect::<Vec<_>>()}
      </ul>
    </div>
  }
}
