# Hi, I'm Craig 'Craole' Cole 👋

![Rust-powered](https://img.shields.io/badge/Rust-Powered-orange?style=for-the-badge&logo=rust&logoColor=white)
![Data Engineer](https://img.shields.io/badge/Data_Engineering-blue?style=for-the-badge&logo=apachespark&logoColor=white)
![Full Stack](https://img.shields.io/badge/Full_Stack-blueviolet?style=for-the-badge&logo=stackexchange&logoColor=white)
![Systems](https://img.shields.io/badge/Systems-green?style=for-the-badge&logo=linux&logoColor=white)

## About Me

Music is my first love. It shaped most of my adult life, long before software did. I played keyboards, guitar, and bass with the church choir and band as a teenager, but bass is the one I fell for and chose as my primary instrument. By my late teens I'd formed my own singing group.

For fifteen to twenty years, music was the center of my life. I played and worked with Skygrass (formerly Blu Grass in the Sky), No-Maddz, Stone Dub, Protoje & The Indiggnation, and BLACK as COLE, a reggae/neo-soul/jazz fusion project I founded to give a collective feeling a shape. As founder, I flexed every creative and production skill I had, wearing hats from manager to producer, web developer, videographer/director, and photographer, experience that quietly prepared me for everything that came later. My creative instincts have always run past music into performance, writing, design, and photography: anything that turns an idea into something real.

Technology was always there too, a steady countermelody running alongside the music. My first job was teaching Microsoft Office to working professionals. From there I moved into computer repair, started IT studies at Northern Caribbean University, and took a shift role in WINDALCO's Kirkvine powerhouse lab. It was chemistry work, not tech, but it paid me to work well past midnight and taught me I do my best thinking outside a 9-to-5 rhythm. Later, I studied Music Performance (Jazz) at the Edna Manley College of the Visual and Performing Arts while holding down full-time Tier 1 networking support, working nights to make both worlds fit.

Eventually I made a deliberate choice to step back from professional music and move into business process outsourcing, starting as an agent and earning a promotion into training and development within three months. From there, my path split three ways: BPO (support, learning and development, business intelligence), TEFL (freelance and with Fluentbe), and now software, data, and AI engineering.

None of it feels like dissonance. It's the same motif, just returning in a new arrangement: learn a system deeply, communicate clearly, build something useful. What moved me about music was never just the performance itself, it was watching it land, seeing a room change because of something I helped create. That's the same thing that pulls me toward this work now: an idea, built into something real, that actually adds value to someone's life. Code is just another instrument for the same purpose.

## Professional Background

### English as a Second Language Specialist

**Freelance and Fluentbe · July 2021 – Present**

- Earned TEFL certification and joined Fluentbe in May 2022; continue to freelance independently, though at a reduced pace.
- Delivered 5,000+ remote learning sessions to roughly 1,800 students across 30+ nationalities as an independent freelancer.
- Prepared learners for IELTS, TOEFL, job interviews, presentations, and high-stakes business meetings.
- Built structured, goal-oriented learning environments through targeted coaching and feedback.

### Business Intelligence Specialist & Training/Quality Coordinator

**NKCS · June 2019 – February 2021**

- Designed, implemented, and maintained BI tools and CRM systems across departments.
- Analyzed business requirements, operational processes, and KPIs to identify improvement areas.
- Launched and oversaw a Training and Quality department during startup operations.
- Designed and delivered leadership, skills, BI-tool, and CRM-system training programs.

### Learning & Development Officer

**HGS · October 2016 – June 2019**

- Promoted from process trainer into leadership and professional development.
- Supported design, testing, implementation, and local rollout of a global LMS.
- Built development programs aligned to organizational goals and identified training gaps.
- Trained leaders and brand advocates in leadership, facilitation, Microsoft Office, customer experience, and sales strategy.
- Contributed to the technical and creative design of forms, dashboards, and print media.

### Process Trainer / Technical Support Brand Advocate

**HGS · September 2015 – January 2017**

- Started as an agent and was promoted to process trainer within three months.
- Coached production teams through targeted training and quality-improvement initiatives.
- Participated in Kaizen events to reduce technical-support error rates.
- Supported Internet, VoIP, and networking issues for Global Capacity and Megapath customers.
- Maintained a 98% matriculation-to-production rate across five training cohorts.



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
- `assets/` — servable static assets: images, project media, audio, fonts, icons, and the favicon.
- `content/assets/` — private authored metadata and prose definitions; the `content/` crate validates these,
  exports SQLite/static data, and creates draft templates.
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

Create draft content with `content new`:

```sh
cargo run -p content -- new project my-project
cargo run -p content -- new post project-build-log
cargo run -p content -- new media portfolio-screenshot
```

Sync validated content into the local SQLite database with one command:

```sh
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db
```

Export static-friendly JSON data for the fallback path:

```sh
cargo run -p content -- export-json . dist/data
```

Generate a minimal static fallback site:

```sh
cargo run -p content -- export-static . dist
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
