use crate::prelude::*;

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
          logo: "icons/logos/rusty.svg",
          link: "https://www.rust-lang.org/",
        },
        Tech {
          name: "Shell",
          logo: "icons/logos/bash.svg",
          link: "https://pubs.opengroup.org/onlinepubs/9799919799/",
        },
        Tech {
          name: "Python",
          logo: "icons/logos/python.svg",
          link: "https://www.python.org/",
        },
        Tech {
          name: "Zig",
          logo: "icons/logos/zig.svg",
          link: "https://ziglang.org/",
        },
      ],
    },
    Stack {
      title: "Web",
      technologies: vec![
        Tech {
          name: "Actix",
          logo: "icons/logos/actix.svg",
          link: "https://actix.rs/",
        },
        Tech {
          name: "Axum",
          logo: "icons/logos/tokio.svg",
          link: "https://github.com/tokio-rs/axum",
        },
        Tech {
          name: "HTMX",
          logo: "icons/logos/htmx.png",
          link: "https://htmx.org/",
        },
        Tech {
          name: "Leptos",
          logo: "icons/logos/leptos.ico",
          link: "https://leptos.dev/",
        },
        Tech {
          name: "Tailwind",
          logo: "icons/logos/tailwind-blue.svg",
          link: "https://tailwindcss.com/",
        },
      ],
    },
    Stack {
      title: "Data Engineering",
      technologies: vec![
        // Tech {
        //   name: "Apache Spark",
        //   logo: "icons/logos/spark.svg",
        //   link: "https://spark.apache.org/",
        // },
        // Tech {
        //   name: "Databricks",
        // logo: "icons/logos/deltalake.svg",
        //   link: "https://www.databricks.com/",
        // },
        Tech {
          name: "DeltaLake",
          logo: "icons/logos/deltalake.svg",
          link: "https://delta.io/",
        },
        Tech {
          name: "SurrealDB",
          logo: "icons/logos/surrealdb.png",
          link: "https://delta.io/",
        },
        Tech {
          name: "Neo4j",
          logo: "icons/logos/neo4j-flat.svg",
          link: "https://neo4j.com/",
        },
        Tech {
          name: "PostgreSQL",
          logo: "icons/logos/postgresql.svg",
          link: "https://www.postgresql.org/",
        },
        Tech {
          name: "SQLite",
          logo: "icons/logos/SQLite.svg",
          link: "https://www.sqlite.org/",
        },
      ],
    },
    Stack {
      title: "DevOps & Systems",
      technologies: vec![
        Tech {
          name: "Git",
          logo: "icons/logos/git.svg",
          link: "https://git-scm.com/",
        },
        Tech {
          name: "GitHub",
          logo: "icons/logos/github.svg",
          link: "https://github.com/",
        },
        Tech {
          name: "NixOS",
          logo: "icons/logos/nix.svg",
          link: "https://nixos.org/",
        },
        Tech {
          name: "Raspberry Pi",
          logo: "icons/logos/raspberry.svg",
          link: "https://www.raspberrypi.com/",
        },
        Tech {
          name: "Windows",
          logo: "icons/logos/windows.svg",
          link: "https://www.microsoft.com/windows",
        },
      ],
    },
    Stack {
      title: "Editors",
      technologies: vec![
        Tech {
          name: "Helix",
          logo: "icons/logos/helix.svg",
          link: "https://helix-editor.com/",
        },
        Tech {
          name: "Typst",
          logo: "icons/logos/typst.svg",
          link: "https://typst.app/",
        },
        Tech {
          name: "VS Code",
          logo: "icons/logos/vscode.svg",
          link: "https://code.visualstudio.com/",
        },
        Tech {
          name: "Zed",
          logo: "icons/logos/zed.webp",
          link: "https://zed.dev/",
        },
      ],
    },
    Stack {
      title: "Terminal",
      technologies: vec![
        Tech {
          name: "Bash",
          logo: "icons/logos/bash.svg",
          link: "https://www.gnu.org/software/bash/",
        },
        Tech {
          name: "PowerShell",
          logo: "icons/logos/powershell.svg",
          link: "https://github.com/PowerShell/PowerShell",
        },
        Tech {
          name: "Starship",
          logo: "icons/logos/starship.svg",
          link: "https://starship.rs/",
        },
        Tech {
          name: "Oh My Posh",
          logo: "icons/logos/ohmyposh.svg",
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
    <div class="p-5 rounded-lg border bg-slate-50 border-slate-200 dark:bg-slate-800/30 dark:border-slate-700/50">
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
      class="flex gap-2 items-center py-2 px-3 bg-white rounded-lg border transition-all hover:border-blue-400 hover:shadow-md hover:scale-105 group border-slate-300 dark:bg-slate-800 dark:border-slate-600 dark:hover:border-blue-500"
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
    <div class="p-6 rounded-xl border transition-all duration-300 hover:shadow-lg bg-white/80 border-slate-200 dark:bg-slate-800/50 dark:border-slate-700/50">
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
