use crate::prelude::Icon;

//╔═══════════════════════════════════════════════════════════╗
//║ Types                                                     ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Clone)]
pub struct Tech {
  pub name: &'static str,
  pub logo: Icon,
  pub link: &'static str,
}

#[derive(Clone)]
pub struct Stack {
  pub title: &'static str,
  pub technologies: Vec<Tech>,
}

#[derive(Clone)]
pub struct Area {
  pub icon: &'static str,
  pub title: &'static str,
  pub points: Vec<&'static str>,
}

//╔═══════════════════════════════════════════════════════════╗
//║ Data                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub fn stacks() -> Vec<Stack> {
  vec![
    Stack {
      title: "Languages & Core",
      technologies: vec![
        Tech {
          name: "Rust",
          logo: Icon::new_library("Rust"),
          link: "https://www.rust-lang.org/",
        },
        Tech {
          name: "Shell",
          logo: Icon::new_custom("icons/logos/bash.svg"),
          link: "https://pubs.opengroup.org/onlinepubs/9799919799/",
        },
        Tech {
          name: "Python",
          logo: Icon::new_custom("icons/logos/python.svg"),
          link: "https://www.python.org/",
        },
        Tech {
          name: "Zig",
          logo: Icon::new_custom("icons/logos/zig.svg"),
          link: "https://ziglang.org/",
        },
      ],
    },
    Stack {
      title: "Web",
      technologies: vec![
        Tech {
          name: "Actix",
          logo: Icon::new_library("Actix").with_class("dark:invert dark:hue-rotate-180"),
          link: "https://actix.rs/",
        },
        Tech {
          name: "Axum",
          logo: Icon::new_custom("icons/logos/tokio.svg")
            .with_class("dark:invert dark:hue-rotate-180"),
          link: "https://github.com/tokio-rs/axum",
        },
        Tech {
          name: "Htmx",
          logo: Icon::new_library("Htmx").with_class("dark:invert dark:hue-rotate-180"),
          link: "https://htmx.org/",
        },
        Tech {
          name: "Leptos",
          logo: Icon::new_custom("icons/logos/leptos.ico"),
          link: "https://leptos.dev/",
        },
        Tech {
          name: "Tailwind",
          logo: Icon::new_custom("icons/logos/tailwind-blue.svg"),
          link: "https://tailwindcss.com/",
        },
      ],
    },
    Stack {
      title: "Data Engineering",
      technologies: vec![
        Tech {
          name: "DeltaLake",
          logo: Icon::new_custom("icons/logos/deltalake.svg"),
          link: "https://delta.io/",
        },
        Tech {
          name: "SurrealDB",
          logo: Icon::new_custom("icons/logos/surrealdb.png"),
          link: "https://surrealdb.com/",
        },
        Tech {
          name: "Neo4j",
          logo: Icon::new_custom("icons/logos/neo4j-flat.svg"),
          link: "https://neo4j.com/",
        },
        Tech {
          name: "PostgreSQL",
          logo: Icon::new_custom("icons/logos/postgresql.svg"),
          link: "https://www.postgresql.org/",
        },
        Tech {
          name: "SQLite",
          logo: Icon::new_custom("icons/logos/SQLite.svg"),
          link: "https://www.sqlite.org/",
        },
      ],
    },
    Stack {
      title: "DevOps & Systems",
      technologies: vec![
        Tech {
          name: "Git",
          // Using custom SVG to preserve original brand colors
          logo: Icon::new_custom("icons/logos/git.svg"),
          link: "https://git-scm.com/",
        },
        Tech {
          name: "GitHub",
          logo: Icon::new_library("Github"),
          link: "https://github.com/",
        },
        Tech {
          name: "NixOS",
          logo: Icon::new_custom("icons/logos/nix.svg"),
          link: "https://nixos.org/",
        },
        Tech {
          name: "Raspberry Pi",
          logo: Icon::new_custom("icons/logos/raspberry.svg"),
          link: "https://www.raspberrypi.com/",
        },
        Tech {
          name: "Windows",
          logo: Icon::new_custom("icons/logos/windows.svg"),
          link: "https://www.microsoft.com/windows",
        },
      ],
    },
    Stack {
      title: "Editors",
      technologies: vec![
        Tech {
          name: "Helix",
          logo: Icon::new_custom("icons/logos/helix.svg"),
          link: "https://helix-editor.com/",
        },
        Tech {
          name: "Typst",
          logo: Icon::new_custom("icons/logos/typst.svg"),
          link: "https://typst.app/",
        },
        Tech {
          name: "VS Code",
          logo: Icon::new_custom("icons/logos/vscode.svg"),
          link: "https://code.visualstudio.com/",
        },
        Tech {
          name: "Zed",
          logo: Icon::new_custom("icons/logos/zed.svg"),
          link: "https://zed.dev/",
        },
      ],
    },
    Stack {
      title: "Terminal",
      technologies: vec![
        Tech {
          name: "Bash",
          logo: Icon::new_custom("icons/logos/bash.svg"),
          link: "https://www.gnu.org/software/bash/",
        },
        Tech {
          name: "PowerShell",
          logo: Icon::new_custom("icons/logos/powershell.svg"),
          link: "https://github.com/PowerShell/PowerShell",
        },
        Tech {
          name: "Starship",
          logo: Icon::new_custom("icons/logos/starship.svg"),
          link: "https://starship.rs/",
        },
        Tech {
          name: "Oh My Posh",
          logo: Icon::new_custom("icons/logos/ohmyposh.svg"),
          link: "https://ohmyposh.dev/",
        },
      ],
    },
  ]
}

pub fn areas() -> Vec<Area> {
  vec![
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
  ]
}
