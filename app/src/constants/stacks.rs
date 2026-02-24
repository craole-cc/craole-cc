use crate::prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Types                                                     ║
//╚═══════════════════════════════════════════════════════════╝
#[derive(Clone,)]
pub struct Tech {
  pub name : &'static str,
  pub logo : Icon,
  pub link : &'static str,
}

#[derive(Clone,)]
pub struct Stack {
  pub title :        &'static str,
  pub technologies : Vec<Tech,>,
}

#[derive(Clone,)]
pub struct Area {
  pub icon :   &'static str,
  pub title :  &'static str,
  pub points : Vec<&'static str,>,
}

//╔═══════════════════════════════════════════════════════════╗
//║ Data                                                      ║
//╚═══════════════════════════════════════════════════════════╝
pub fn stacks() -> Vec<Stack,> {
  vec![
    Stack {
      title :        "Languages & Core",
      technologies : vec![
        Tech {
          name : "Rust",
          logo : Icons::Rust.get(),
          link : "https://www.rust-lang.org/",
        },
        Tech {
          name : "Shell",
          logo : Icons::Shell.get(),
          link : "https://pubs.opengroup.org/onlinepubs/9799919799/",
        },
        Tech {
          name : "Python",
          logo : Icons::Python.get(),
          link : "https://www.python.org/",
        },
        Tech {
          name : "Zig",
          logo : Icons::Zig.get(),
          link : "https://ziglang.org/",
        },
      ],
    },
    Stack {
      title :        "Web",
      technologies : vec![
        Tech {
          name : "Actix",
          logo : Icons::Actix.get(),
          link : "https://actix.rs/",
        },
        Tech {
          name : "Axum",
          logo : Icons::Axum.get(),
          link : "https://github.com/tokio-rs/axum",
        },
        Tech {
          name : "HTMX",
          logo : Icons::Htmx.get(),
          link : "https://htmx.org/",
        },
        Tech {
          name : "Leptos",
          logo : Icons::Leptos.get(),
          link : "https://leptos.dev/",
        },
        Tech {
          name : "Tailwind",
          logo : Icons::Tailwind.get(),
          link : "https://tailwindcss.com/",
        },
      ],
    },
    Stack {
      title :        "Data Engineering",
      technologies : vec![
        Tech {
          name : "Delta Lake",
          logo : Icons::DeltaLake.get(),
          link : "https://delta.io/",
        },
        Tech {
          name : "SurrealDB",
          logo : Icons::SurrealDb.get(),
          link : "https://surrealdb.com/",
        },
        Tech {
          name : "Neo4j",
          logo : Icons::Neo4j.get(),
          link : "https://neo4j.com/",
        },
        Tech {
          name : "PostgreSQL",
          logo : Icons::PostgreSql.get(),
          link : "https://www.postgresql.org/",
        },
        Tech {
          name : "SQLite",
          logo : Icons::Sqlite.get(),
          link : "https://www.sqlite.org/",
        },
      ],
    },
    Stack {
      title :        "DevOps & Systems",
      technologies : vec![
        Tech {
          name : "Git",
          logo : Icons::Git.get(),
          link : "https://git-scm.com/",
        },
        Tech {
          name : "GitHub",
          logo : Icons::GitHub.get(),
          link : "https://github.com/",
        },
        Tech {
          name : "NixOS",
          logo : Icons::NixOs.get(),
          link : "https://nixos.org/",
        },
        Tech {
          name : "Raspberry Pi",
          logo : Icons::RaspberryPi.get(),
          link : "https://www.raspberrypi.com/",
        },
        Tech {
          name : "Windows",
          logo : Icons::Windows.get(),
          link : "https://www.microsoft.com/windows",
        },
      ],
    },
    Stack {
      title :        "Editors",
      technologies : vec![
        Tech {
          name : "Helix",
          logo : Icons::Helix.get(),
          link : "https://helix-editor.com/",
        },
        Tech {
          name : "Typst",
          logo : Icons::Typst.get(),
          link : "https://typst.app/",
        },
        Tech {
          name : "VS Code",
          logo : Icons::VSCode.get(),
          link : "https://code.visualstudio.com/",
        },
        Tech {
          name : "Zed",
          logo : Icons::Zed.get(),
          link : "https://zed.dev/",
        },
      ],
    },
    Stack {
      title :        "Terminal",
      technologies : vec![
        Tech {
          name : "Bash",
          logo : Icons::Bash.get(),
          link : "https://www.gnu.org/software/bash/",
        },
        Tech {
          name : "PowerShell",
          logo : Icons::PowerShell.get(),
          link : "https://github.com/PowerShell/PowerShell",
        },
        Tech {
          name : "Starship",
          logo : Icons::Starship.get(),
          link : "https://starship.rs/",
        },
        Tech {
          name : "Oh My Posh",
          logo : Icons::OhMyPosh.get(),
          link : "https://ohmyposh.dev/",
        },
      ],
    },
  ]
}

pub fn areas() -> Vec<Area,> {
  vec![
    Area {
      icon :   "🌐",
      title :  "Full-Stack Development",
      points : vec![
        "Web applications with Rust backends (Axum, Actix) + modern frontends (HTMX, SPA \
         frameworks)",
        "End-to-end solutions from database to UI",
      ],
    },
    Area {
      icon :   "📊",
      title :  "Data Engineering",
      points : vec![
        "High-performance data pipelines with Rust & Delta Lake",
        "Graph databases (Neo4j) for complex relationship modeling",
        "Analytics infrastructure that scales",
      ],
    },
    Area {
      icon :   "⚙️",
      title :  "Systems Utilities",
      points : vec![
        "Command-line tools that solve everyday problems elegantly",
        "Cross-platform applications (Windows, Linux, macOS)",
        "Developer productivity tools",
      ],
    },
    Area {
      icon :   "🔧",
      title :  "Infrastructure",
      points : vec![
        "Declarative NixOS configurations for reproducible environments",
        "Infrastructure-as-code and automation",
        "Self-hosted solutions",
      ],
    },
  ]
}
