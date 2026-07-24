use crate::prelude::{Icon, icons::*};

// -- Types ─────────────────────────────────────────────────────────────────────

pub struct Stack {
  pub title: &'static str,
  pub icons: &'static [fn() -> Icon],
}

pub struct Area {
  pub emoji: &'static str,
  pub title: &'static str,
  pub points: &'static [&'static str],
}

// -- Macro ─────────────────────────────────────────────────────────────────────

macro_rules! icons {
  ($($m:ident),+ $(,)?) => { &[$($m::default),+] }
}

// -- Data ──────────────────────────────────────────────────────────────────────

pub const STACKS: &[Stack] = &[
  Stack {
    title: "Languages",
    icons: icons![bash, powershell, python, rust, zig],
  },
  Stack {
    title: "Web",
    icons: icons![actix, axum, htmx, leptos, tailwind],
  },
  Stack {
    title: "Data",
    icons: icons![sqlite],
  },
  Stack {
    title: "DevOps",
    icons: icons![git, github],
  },
  Stack {
    title: "Systems",
    icons: icons![linux, nix, raspberry_pi, windows],
  },
  Stack {
    title: "Editors",
    icons: icons![helix, zed, vscode, typst],
  },
];

pub const AREAS: &[Area] = &[
  Area {
    emoji: "🌐",
    title: "Full-Stack Development",
    points: &[
      "Web applications with Rust backends (Axum, Actix) and modern frontends (Leptos, HTMX)",
      "End-to-end solutions from database schema to UI component",
    ],
  },
  Area {
    emoji: "📊",
    title: "Data and Business Intelligence",
    points: &[
      "Data workflows that connect business requirements, process metrics, systems, and usable outputs",
      "Validation, deterministic exports, and SQLite-backed application data",
      "Analytics and CRM-focused problem solving grounded in BI experience",
    ],
  },
  Area {
    emoji: "🤖",
    title: "AI and Automation",
    points: &[
      "Local inference and LLM workflows with privacy, cost control, and reproducibility in mind",
      "Agent tooling, model routing, and documented developer automation",
    ],
  },
  Area {
    emoji: "⚙️",
    title: "Systems Utilities",
    points: &[
      "Command-line tools and developer workflows that make complex systems easier to operate",
      "Cross-platform development targeting Windows and Linux",
      "Reproducible Nix environments and Rust toolchains",
    ],
  },
  Area {
    emoji: "🔧",
    title: "Infrastructure",
    points: &[
      "Declarative NixOS configurations for reproducible environments",
      "Infrastructure-as-code and self-hosted services",
      "Homelab automation with Raspberry Pi and Ansible",
    ],
  },
];
