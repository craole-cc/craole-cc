use crate::_prelude::*;

//╔═══════════════════════════════════════════════════════════╗
//║ Types                                                     ║
//╚═══════════════════════════════════════════════════════════╝

#[derive(Clone,)]
pub struct Tech {
  pub name : &'static str,
  pub icon : Icons,
  pub link : &'static str,
}

impl Tech {
  /// Resolve the Icon for rendering. Link and label come from the registry,
  /// so callers don't need to repeat them — but the link field here can
  /// override the registry default if needed (e.g. a specific docs page).
  pub fn icon(&self,) -> Icon { self.icon.get() }
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
          icon : Icons::Rust,
          link : "https://www.rust-lang.org/",
        },
        Tech {
          name : "Shell",
          icon : Icons::Posix,
          link : "https://pubs.opengroup.org/onlinepubs/9799919799/",
        },
        Tech {
          name : "Python",
          icon : Icons::Python,
          link : "https://www.python.org/",
        },
        Tech {
          name : "Zig",
          icon : Icons::Zig,
          link : "https://ziglang.org/",
        },
      ],
    },
    Stack {
      title :        "Web",
      technologies : vec![
        Tech {
          name : "Actix",
          icon : Icons::Actix,
          link : "https://actix.rs/",
        },
        Tech {
          name : "Axum",
          icon : Icons::Axum,
          link : "https://github.com/tokio-rs/axum",
        },
        Tech {
          name : "HTMX",
          icon : Icons::Htmx,
          link : "https://htmx.org/",
        },
        Tech {
          name : "Leptos",
          icon : Icons::Leptos,
          link : "https://leptos.dev/",
        },
        Tech {
          name : "Tailwind",
          icon : Icons::Tailwind,
          link : "https://tailwindcss.com/",
        },
      ],
    },
    Stack {
      title :        "Data Engineering",
      technologies : vec![
        Tech {
          name : "Delta Lake",
          icon : Icons::DeltaLake,
          link : "https://delta.io/",
        },
        Tech {
          name : "SurrealDB",
          icon : Icons::SurrealDb,
          link : "https://surrealdb.com/",
        },
        Tech {
          name : "Neo4j",
          icon : Icons::Neo4j,
          link : "https://neo4j.com/",
        },
        Tech {
          name : "PostgreSQL",
          icon : Icons::PostgreSql,
          link : "https://www.postgresql.org/",
        },
        Tech {
          name : "SQLite",
          icon : Icons::Sqlite,
          link : "https://www.sqlite.org/",
        },
      ],
    },
    Stack {
      title :        "DevOps & Systems",
      technologies : vec![
        Tech {
          name : "Git",
          icon : Icons::Git,
          link : "https://git-scm.com/",
        },
        Tech {
          name : "GitHub",
          icon : Icons::GitHub,
          link : "https://github.com/",
        },
        Tech {
          name : "NixOS",
          icon : Icons::NixOs,
          link : "https://nixos.org/",
        },
        Tech {
          name : "Raspberry Pi",
          icon : Icons::RaspberryPi,
          link : "https://www.raspberrypi.com/",
        },
        Tech {
          name : "Windows",
          icon : Icons::Windows,
          link : "https://www.microsoft.com/windows",
        },
      ],
    },
    Stack {
      title :        "Editors",
      technologies : vec![
        Tech {
          name : "Helix",
          icon : Icons::Helix,
          link : "https://helix-editor.com/",
        },
        Tech {
          name : "Typst",
          icon : Icons::Typst,
          link : "https://typst.app/",
        },
        Tech {
          name : "VS Code",
          icon : Icons::VSCode,
          link : "https://code.visualstudio.com/",
        },
        Tech {
          name : "Zed",
          icon : Icons::Zed,
          link : "https://zed.dev/",
        },
      ],
    },
    Stack {
      title :        "Terminal",
      technologies : vec![
        Tech {
          name : "Bash",
          icon : Icons::Bash,
          link : "https://www.gnu.org/software/bash/",
        },
        Tech {
          name : "PowerShell",
          icon : Icons::PowerShell,
          link : "https://github.com/PowerShell/PowerShell",
        },
        Tech {
          name : "Starship",
          icon : Icons::Starship,
          link : "https://starship.rs/",
        },
        Tech {
          name : "Oh My Posh",
          icon : Icons::OhMyPosh,
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
