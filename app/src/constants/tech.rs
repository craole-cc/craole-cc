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
// Maps project/media tag strings to branded `default()` icons so that
// tech-stack sections can display the custom icon system rather than raw
// text tags.  Uses `default()` variants to guarantee a `.brand-*` colour
// class is present for the SCSS hover effect to work.

#[must_use]
pub fn icon_for_tag(tag : &str,) -> Option<Icon,> {
  match tag {
    // Languages
    | "Rust" | "rust" => Some(rust::default(),),
    | "Python" | "python" => Some(python::default(),),
    | "Zig" | "zig" => Some(zig::default(),),
    | "Bash" | "bash" | "Shell" | "ShellScript" => Some(bash::default(),),
    | "PowerShell" | "powershell" => Some(powershell::default(),),
    | "Nushell" | "nushell" => Some(nushell::default(),),
    | "JavaScript" | "javascript" | "JS" | "js" => Some(javascript::default(),),
    | "TypeScript" | "typescript" | "TS" | "ts" => Some(typescript::default(),),
    | "POSIX" | "posix" => Some(posix::default(),),

    // Web
    | "Leptos" | "leptos" => Some(leptos::default(),),
    | "HTMX" | "htmx" => Some(htmx::default(),),
    | "Tailwind" | "tailwind" | "Tailwind CSS" | "tailwindcss" => Some(tailwind::default(),),
    | "Axum" | "axum" => Some(axum::default(),), // no Leptos icon — use local SVG

    // Data
    | "SQLite" | "sqlite" => Some(sqlite::default(),),
    | "PostgreSQL" | "postgresql" => Some(postgresql::default(),),
    | "Neo4j" | "neo4j" => Some(neo4j::default(),),
    | "Delta Lake" | "DeltaLake" | "delta-lake" => Some(deltalake::default(),),
    | "SurrealDB" | "surrealdb" => Some(surrealdb::default(),),

    // DevOps / OS
    | "Git" | "git" => Some(git::default(),),
    | "GitHub" | "github" => Some(github::default(),),
    | "GitLab" | "gitlab" => Some(gitlab::default(),),
    | "Docker" | "docker" => Some(docker::default(),),
    | "Kubernetes" | "kubernetes" | "k8s" => Some(kubernetes::default(),),
    | "Linux" | "linux" => Some(linux::default(),),
    | "Nix" | "nix" | "NixOS" | "nixos" => Some(nix::default(),),
    | "Ansible" | "ansible" => Some(ansible::default(),),
    | "Terraform" | "terraform" => Some(terraform::default(),),
    | "Raspberry Pi" | "raspberry-pi" | "RaspberryPi" => Some(raspberry_pi::default(),),
    | "Windows" | "windows" => Some(windows::default(),),

    // Editors / tools
    | "Helix" | "helix" => Some(helix::default(),),
    | "Zed" | "zed" => Some(zed::default(),),
    | "VS Code" | "vscode" | "VSCode" => Some(vscode::default(),),
    | "Typst" | "typst" => Some(typst::default(),),
    | "Starship" | "starship" => Some(starship::default(),),

    | _ => None,
  }
}
