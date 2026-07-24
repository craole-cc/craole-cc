# Hi, I'm Craig 'Craole' Cole 👋

![Rust-powered](https://img.shields.io/badge/Rust-Powered-orange?style=for-the-badge&logo=rust&logoColor=white)
![Data Engineer](https://img.shields.io/badge/Data_Engineering-blue?style=for-the-badge&logo=apachespark&logoColor=white)
![Full Stack](https://img.shields.io/badge/Full_Stack-blueviolet?style=for-the-badge&logo=stackexchange&logoColor=white)
![Systems](https://img.shields.io/badge/Systems-green?style=for-the-badge&logo=linux&logoColor=white)

## 👤 About Me

Before software, I spent most of my adult life making music. I started on keyboards and guitar as a
teenager, then settled on bass. Church choir and band gave me an early place to perform; I also
formed a singing group while I was still a teenager. Music stayed at the centre of my life for the
next 15–20 years, including long-term work with **Skygrass** (formerly **Blu Grass in the Sky**),
**No-maddz**, **Stone Dub**, **Protoje & The Indiggnation**, and **BLACK as COLE**. I also played with
many other Jamaican acts over the years.

With BLACK as COLE, I released the single **“Musical Romance.”** My creative interests have always
extended beyond music into the arts generally: performing, literary work, design, photography, and
anything that gives an idea a form.

Technology was present early too. My first formal job was a short-term role teaching Microsoft Office
to professionals. I then repaired computers as a technician and worked in the powerhouse laboratory
at WINDALCO’s Kirkvine operation. I started IT studies at Northern Caribbean University, but funding
interrupted that path. Later, I studied Music Performance (Jazz) at the **Edna Manley College of the
Visual and Performing Arts** while working full time in Tier 1 networking support.

Eventually I made a deliberate decision to pause music professionally. I moved fully into business
process outsourcing, then into learning and development, business intelligence, remote teaching, and
now Rust-first software, data, and AI engineering. Development feels like a natural next chapter
because it brings together design, creativity, and structured problem-solving. It also gives me room
for sustained concentration and project-based, asynchronous work rather than requiring a rigid 9-to-5
rhythm. In that sense, the work is closer to music production than to a conventional office routine.

The thread through all of it is the same: learn a system deeply, communicate clearly, and make
something useful.

> **Code is another instrument of expression through structure** 🎵⚙️

## 💼 Professional Background

My early working life combined teaching and technical roles with music:

- Taught Microsoft Office to working professionals in my first formal short-term role.
- Worked as a computer technician, repairing and supporting people's computers.
- Worked in the powerhouse laboratory at WINDALCO’s Kirkvine operation.
- Began IT studies at Northern Caribbean University and later studied Music Performance (Jazz) at
  the Edna Manley College of the Visual and Performing Arts; financial constraints shaped the timing
  and path of that education.
- Maintained full-time Tier 1 networking-support work while studying music, before pausing music
  professionally and moving fully into BPO and Learning & Development.

- **English as a Second Language Specialist** — freelance and remote, July 2021–present; 5,000+
  sessions, approximately 1,800 students, and 30+ nationalities.
- **Business Intelligence Specialist & Training / Quality Coordinator** — NKCS, June 2019–February
  2021; BI tools, CRM systems, requirements analysis, process improvement, and training operations.
- **Learning & Development Officer** — HGS, October 2016–June 2019; learning-management systems,
  development programs, leadership training, dashboards, and technical support.
- **Process Trainer / Technical Support Brand Advocate** — HGS, September 2015–January 2017;
  technical-support coaching, quality improvement, networking support, and a 98% matriculation-to-
  production rate across five cohorts.

## 🌐 About this repository

`craole-cc/craole-cc` is both my GitHub profile README and the source for the [craole.cc](https://craole.cc): a Rust-first portfolio site that showcases engineering craft,
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
- `contentctl/` — CLI for validating content, exporting a SQLite seed script, and creating draft
  content templates.
- `database/migrations/` — durable SQLite schema and baseline seed data.
- `scripts/ci.sh` — local/CI quality gate for database setup, content validation, SQLx, clippy,
  and tests.

## 🛠️ What I Build

### 🌐 Full-Stack Development

- Rust web applications with Leptos, Axum, server rendering, and WASM hydration.
- End-to-end systems from SQLite schema and SQLx-checked queries to accessible UI.
- Local-first content workflows that can validate, export, and support a low-cost fallback path.

### 📊 Data and Business Intelligence

- Data workflows that connect business requirements, process metrics, systems, and usable outputs.
- Content and application data pipelines with validation, deterministic exports, and SQLite sync.
- Analytics and CRM-focused problem solving grounded in prior BI and operations experience.

### 🤖 AI and Automation

- Local inference and LLM workflows that prioritize privacy, cost control, and reproducibility.
- Agent tooling, model routing, developer automation, and documented operational workflows.

### 🔧 Infrastructure and Systems

- Reproducible Nix development environments and Rust toolchains.
- Linux services, reverse proxies, deployment artifacts, backups, monitoring, and recovery.
- Command-line tools and developer workflows that make complex systems easier to operate.

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

### Data & Business Systems

![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)
![SQL](https://img.shields.io/badge/SQL-336791?style=for-the-badge&logo=postgresql&logoColor=white)

<!-- Professional BI/CRM experience includes tools whose exact versions and depth are documented in private career notes. -->

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
leptoswatch
```

Open `http://127.0.0.1:3000` once the server starts. If you need a different port, pass it as the
first argument; for example, `leptoswatch 3005` starts the site on `http://127.0.0.1:3005` and uses
`3006` for hot-reload. The shortcut checks the site and reload ports first and offers to kill stale
listeners before starting `cargo leptos watch`.

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

Create draft content with `contentctl new`:

```sh
cargo run -p contentctl -- new project my-project
cargo run -p contentctl -- new post project-build-log
cargo run -p contentctl -- new media portfolio-screenshot
```

Sync validated content into the local SQLite database with one command:

```sh
cargo run -p contentctl -- sync-db . sqlite://database/data/portfolio.db
```

Export static-friendly JSON data for the fallback path:

```sh
cargo run -p contentctl -- export-json . dist/data
```

Generate a minimal static fallback site:

```sh
cargo run -p contentctl -- export-static . dist
```

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
