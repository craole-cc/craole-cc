use crate::prelude::{
  Icon,
  icons::*,
};

// -- Types ─────────────────────────────────────────────────────────────────────

pub struct Stack {
  pub title : &'static str,
  pub icons : &'static [fn() -> Icon],
}

pub struct Area {
  pub emoji :  &'static str,
  pub title :  &'static str,
  pub points : &'static [&'static str],
}

// -- Macro ─────────────────────────────────────────────────────────────────────

macro_rules! icons {
  ($($m:ident),+ $(,)?) => { &[$($m::default),+] }
}

// -- Data ──────────────────────────────────────────────────────────────────────

pub const STACKS : &[Stack] = &[
  Stack {
    title : "Languages",
    icons : icons![bash, powershell, python, rust, zig],
  },
  Stack {
    title : "Web",
    icons : icons![actix, axum, htmx, leptos, tailwind],
  },
  Stack {
    title : "Data",
    icons : icons![deltalake, surrealdb, neo4j, postgresql, sqlite],
  },
  Stack {
    title : "DevOps",
    icons : icons![git, github, gitlab, docker],
  },
  Stack {
    title : "Systems",
    icons : icons![linux, nix, raspberry_pi, windows],
  },
  Stack {
    title : "Editors",
    icons : icons![helix, zed, vscode, typst],
  },
];

pub const AREAS : &[Area] = &[
  Area {
    emoji :  "🌐",
    title :  "Full-Stack Development",
    points : &[
      "Web applications with Rust backends (Axum, Actix) and modern frontends (Leptos, HTMX)",
      "End-to-end solutions from database schema to UI component",
    ],
  },
  Area {
    emoji :  "📊",
    title :  "Data Engineering",
    points : &[
      "High-performance pipelines with Rust and Delta Lake",
      "Graph databases (Neo4j) for complex relationship modelling",
      "Analytics infrastructure built to scale",
    ],
  },
  Area {
    emoji :  "⚙️",
    title :  "Systems Utilities",
    points : &[
      "Command-line tools that solve everyday problems elegantly",
      "Cross-platform applications targeting Windows, Linux, and macOS",
      "Developer productivity and automation tooling",
    ],
  },
  Area {
    emoji :  "🔧",
    title :  "Infrastructure",
    points : &[
      "Declarative NixOS configurations for reproducible environments",
      "Infrastructure-as-code and self-hosted services",
      "Homelab automation with Raspberry Pi and Ansible",
    ],
  },
];

// -- Tag → Icon mapping ────────────────────────────────────────────────────────
//
// Maps project/media tag strings to branded `filled()` icons so that
// tech-stack sections can display the custom icon system rather than raw
// text tags.  Uses `filled()` variants to guarantee a `.brand-*` colour
// class is present for the SCSS hover effect to work.

#[must_use]
pub fn icon_for_tag(tag : &str,) -> Option<Icon,> {
  match tag {
    // Languages
    | "Rust" | "rust" => Some(rust::filled(),),
    | "Python" | "python" => Some(python::filled(),),
    | "Zig" | "zig" => Some(zig::filled(),),
    | "Bash" | "bash" | "Shell" | "ShellScript" => Some(bash::filled(),),
    | "PowerShell" | "powershell" => Some(powershell::filled(),),
    | "Nushell" | "nushell" => Some(nushell::filled(),),
    | "JavaScript" | "javascript" | "JS" | "js" => Some(javascript::filled(),),
    | "TypeScript" | "typescript" | "TS" | "ts" => Some(typescript::filled(),),
    | "POSIX" | "posix" => Some(posix::filled(),),

    // Web
    | "Leptos" | "leptos" => Some(leptos::filled(),),
    | "HTMX" | "htmx" => Some(htmx::filled(),),
    | "Tailwind" | "tailwind" | "Tailwind CSS" | "tailwindcss" => Some(tailwind::filled(),),
    | "Axum" | "axum" => Some(axum::local(),), // no Leptos icon — use local SVG

    // Data
    | "SQLite" | "sqlite" => Some(sqlite::filled(),),
    | "PostgreSQL" | "postgresql" => Some(postgresql::filled(),),
    | "Neo4j" | "neo4j" => Some(neo4j::local(),),
    | "Delta Lake" | "DeltaLake" | "delta-lake" => Some(deltalake::local(),),
    | "SurrealDB" | "surrealdb" => Some(surrealdb::filled(),),

    // DevOps / OS
    | "Git" | "git" => Some(git::filled(),),
    | "GitHub" | "github" => Some(github::filled(),),
    | "GitLab" | "gitlab" => Some(gitlab::filled(),),
    | "Docker" | "docker" => Some(docker::filled(),),
    | "Kubernetes" | "kubernetes" | "k8s" => Some(kubernetes::filled(),),
    | "Linux" | "linux" => Some(linux::filled(),),
    | "Nix" | "nix" | "NixOS" | "nixos" => Some(nix::filled(),),
    | "Ansible" | "ansible" => Some(ansible::filled(),),
    | "Terraform" | "terraform" => Some(terraform::filled(),),
    | "Raspberry Pi" | "raspberry-pi" | "RaspberryPi" => Some(raspberry_pi::filled(),),
    | "Windows" | "windows" => Some(windows::filled(),),

    // Editors / tools
    | "Helix" | "helix" => Some(helix::filled(),),
    | "Zed" | "zed" => Some(zed::filled(),),
    | "VS Code" | "vscode" | "VSCode" => Some(vscode::filled(),),
    | "Typst" | "typst" => Some(typst::filled(),),
    | "Starship" | "starship" => Some(starship::filled(),),

    | _ => None,
  }
}
