# Hi, I'm Craig 'Craole' Cole 👋

![Rust-powered](https://img.shields.io/badge/Rust-Powered-orange?style=for-the-badge&logo=rust&logoColor=white)
![Data Engineer](https://img.shields.io/badge/Data_Engineering-blue?style=for-the-badge&logo=apachespark&logoColor=white)
![Full Stack](https://img.shields.io/badge/Full_Stack-blueviolet?style=for-the-badge&logo=stackexchange&logoColor=white)
![Systems](https://img.shields.io/badge/Systems-green?style=for-the-badge&logo=linux&logoColor=white)

## 👤 About Me

I build **full-stack applications, data infrastructure, and systems utilities**, usually with
**Rust** at the core. From frontend interfaces to backend pipelines to command-line tools, I'm
exploring the entire spectrum of what's possible with modern systems programming.

With roots in **music production** (bassist, singer, producer) and 8+ years in **Learning &
Development** (including TEFL tutoring for professionals), I've always been driven by
**expression**. The alias **"Craole"** embodies this — a fusion of my Caribbean heritage and
creative identity, reminding me that _**I have something to say**_.

Music remains part of my voice, but **code has expanded my range**. Every system I build, every
pipeline I design, every utility I craft — it's all **expression through structure**, blending the
precision and creativity I honed as a musician with the power of modern systems programming.

> **Code is another instrument of expression through structure** 🎵⚙️

## 🌐 About this repository

`craole-cc/craole-cc` is both my GitHub profile README and the source for the next version of
[craole.cc](https://craole.cc): a Rust-first portfolio site that showcases engineering craft,
personal style, and a local-first content workflow.

The site is intentionally built on two tracks:

- **Robust path:** a Leptos + Axum + SQLite full-stack application with server rendering,
  structured content, migrations, SQLx-checked queries, and CI gates.
- **Fallback path:** Git-tracked Markdown/TOML content that can be validated and exported into a
  deterministic SQLite seed script, making a future static snapshot or low-cost host easier to
  support if dynamic hosting is unavailable.

Key implementation pieces:

- `app/` — shared Leptos UI, pages, components, theme, and SQL query wrappers.
- `backend/` — Axum server entry point and database bootstrap.
- `frontend/` — WASM hydration entry point.
- `content/` — portfolio-owned source content: projects, posts, media, and schema docs.
- `contentctl/` — CLI for validating content and exporting a SQLite seed script.
- `database/migrations/` — durable SQLite schema and baseline seed data.
- `scripts/ci.sh` — local/CI quality gate for database setup, content validation, SQLx, clippy,
  and tests.

## 🛠️ What I Build

### 🌐 Full-Stack Development

- Web applications with Rust backends (Axum, Actix) + modern frontends (HTMX, Leptos, SPA frameworks)
- End-to-end solutions from database to UI
- Local-first content and data workflows that can degrade gracefully

### 📊 Data Engineering

- High-performance data pipelines with Rust & Delta Lake
- Graph databases (Neo4j) for complex relationship modeling
- Analytics infrastructure that scales

### ⚙️ Systems Utilities

- Command-line tools that solve everyday problems elegantly
- Cross-platform applications (Windows, Linux, macOS)
- Developer productivity tools

### 🔧 Infrastructure

- Declarative NixOS configurations for reproducible environments
- Infrastructure-as-code and automation
- Self-hosted solutions

## 🧰 Tech Stack

### Languages & Core

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Shell](https://img.shields.io/badge/Shellscript-4EAA25?style=for-the-badge&logo=gnubash&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Zig](https://img.shields.io/badge/Zig-F7A41D?style=for-the-badge&logo=zig&logoColor=white)

### Web

![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=tokio&logoColor=white)
![HTMX](https://img.shields.io/badge/HTMX-3366CC?style=for-the-badge&logo=htmx&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos-EF3939?style=for-the-badge&logo=leptos&logoColor=white)
![Tailwind](https://img.shields.io/badge/tailwind-06B6D4?style=for-the-badge&logo=tailwindcss&logoColor=white)

<!-- ![Actix](https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=actix&logoColor=white) -->

### Data Engineering

![Apache Spark](https://img.shields.io/badge/Apache_Spark-E25A1C?style=for-the-badge&logo=apachespark&logoColor=white)
![Databricks](https://img.shields.io/badge/Databricks-FF3621?style=for-the-badge&logo=databricks&logoColor=white)
![Neo4j](https://img.shields.io/badge/Neo4j-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

<!-- ![Power BI](https://img.shields.io/badge/Power_BI-F2C811?style=for-the-badge&logo=powerbi&logoColor=black) -->
<!-- ![Tableau](https://img.shields.io/badge/Tableau-E97627?style=for-the-badge&logo=tableau&logoColor=white) -->

### DevOps & Systems

![Git](https://img.shields.io/badge/Git-F05032?style=for-the-badge&logo=git&logoColor=white)
![NixOS](https://img.shields.io/badge/NixOS-5277C3?style=for-the-badge&logo=nixos&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=dask&logoColor=white)

<!-- ![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black) -->
<!-- ![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white) -->

### Editors

![Helix](https://img.shields.io/badge/Helix-281733?style=for-the-badge&logo=helix&logoColor=white)
![Typst](https://img.shields.io/badge/Typst-239DAD?style=for-the-badge&logo=typst&logoColor=white)
![VS Code](https://img.shields.io/badge/VSCodium-2F80ED?style=for-the-badge&logo=vscodium&logoColor=white)
![Zed  Editor](https://img.shields.io/badge/Zed-084CCF?style=for-the-badge&logo=zedindustries&logoColor=white)

<!-- ![Neovim](https://img.shields.io/badge/Neovim-57A143?style=for-the-badge&logo=neovim&logoColor=white) -->

### Terminal

![Bash](https://img.shields.io/badge/Bash-4EAA25?style=for-the-badge&logo=gnubash&logoColor=white)
![PowerShell](https://img.shields.io/badge/PowerShell-1c56a1?style=for-the-badge&logo=educative&logoColor=white)
![Starship](https://img.shields.io/badge/Starship-DD0B78?style=for-the-badge&logo=starship&logoColor=white)
![OhMyPosh](https://img.shields.io/badge/OhMyPosh-173448?style=for-the-badge&logo=gnometerminal&logoColor=white)

## 🚀 Quick start

The preferred development path is Nix because it provides the pinned Rust toolchain, `cargo-leptos`,
`sqlx`, `sqlite3`, and supporting tools in one reproducible shell:

```sh
nix develop
cp .env.example .env
./scripts/init-db.rs
cargo leptos watch
```

Open `http://127.0.0.1:3000` once the server starts.

Without Nix, install the Rust toolchain from `rust-toolchain.toml`, then install the required Cargo
CLIs:

```sh
cargo install cargo-leptos
cargo install sqlx-cli --no-default-features --features sqlite
cp .env.example .env
./scripts/init-db.rs
cargo leptos watch
```

For full setup, content authoring, and CI instructions, see [CONTRIBUTING.md](./CONTRIBUTING.md).
For the Git-tracked content format, see [content/SCHEMA.md](./content/SCHEMA.md).

## ✅ Quality gate

Run the same checks used by CI before opening a pull request:

```sh
nix develop --command bash scripts/ci.sh
```

The gate prepares SQLite, validates content, smoke-tests the content seed export, checks SQLx
metadata, runs `cargo check`, runs clippy with warnings denied, and runs the workspace tests.

## 🎯 Philosophy

Code is expression. Just like music, it requires **precision, creativity, and purpose**. Whether I'm
building a web app, designing a data pipeline, or crafting a CLI tool, it's about solving problems
in ways that feel **structured yet innovative**.

From my musical background to teaching to building systems—it's all connected through the desire to
**create and communicate**.

## 🚀 Open to Collaboration

Looking for **Rust-centric projects** across the full spectrum:

- 🌐 **Full-stack applications** with modern Rust frameworks
- 📊 **Data engineering** pipelines and infrastructure
- ⚙️ **Developer tools** and systems utilities
- 🎨 **Creative technical** experiments

## 📫 Let's Connect

[![Email](https://img.shields.io/badge/craig.craole.cole@gmail.com-EA4335?style=for-the-badge&logo=gmail&logoColor=white)](mailto:craig.craole.cole@gmail.com)
[![Twitter](https://img.shields.io/badge/craole-1DA1F2?style=for-the-badge&logo=x&logoColor=white)](https://twitter.com/craole)
[![Portfolio](https://img.shields.io/badge/www.craole.cc-blueviolet?style=for-the-badge&logo=zenbrowser&logoColor=white)](https://craole.cc)

---

**Let's build something impactful together** — solving real problems through elegant, expressive
code 🚀

<!---
craole-cc/craole-cc is a ✨ special ✨ repository because its `README.md` (this file) appears on your GitHub profile.
--->
