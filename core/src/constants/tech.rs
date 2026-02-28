use crate::prelude::{
  Icon,
  icons::*,
};

// ── Types ─────────────────────────────────────────────────────────────────────

pub struct Stack {
  pub title : &'static str,
  pub icons : &'static [fn() -> Icon],
}

pub struct Area {
  pub emoji :  &'static str,
  pub title :  &'static str,
  pub points : &'static [&'static str],
}

// ── Macro ─────────────────────────────────────────────────────────────────────

macro_rules! icons {
  ($($m:ident),+ $(,)?) => { &[$($m::default),+] }
}

// ── Data ──────────────────────────────────────────────────────────────────────

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
