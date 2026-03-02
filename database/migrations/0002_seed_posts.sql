INSERT INTO posts (title, slug, body, excerpt, kind, published_at, tags) VALUES

(
  'Curriculum Vitae',
  'curriculum-vitae',
  '## Summary

Software engineer and data specialist with roots in music production and Learning & Development. I build backend systems, data pipelines, and developer tools — primarily in Rust.

## Experience

### Data & Backend Development *(2024 – Present)*
- Designing data pipelines with Rust and Delta Lake
- Exploring graph databases (Neo4j) and advanced SQL patterns
- Building portfolio site with Leptos + Axum (full-stack Rust)

### Learning & Development Specialist *(8 years)*
- Built training programs and analytics dashboards in Power BI and Tableau
- Managed data-driven decision-making processes across organisations

### TEFL Tutor *(3 years)*
- Teaching professionals and students English communication
- Developing custom learning materials and curricula

## Skills

**Languages:** Rust, SQL, Nix, Bash, PowerShell
**Tools:** Delta Lake, Neo4j, SQLite, NixOS, Git
**Previous:** Power BI, Tableau, Excel

## Education

Self-taught software developer with continuous learning through projects and documentation.',
  'Software engineer and data specialist with roots in music production and Learning & Development.',
  'CV',
  '2026-03-01 09:00:00',
  '["career","rust","data"]'
),

(
  'Why I Chose Rust for Everything',
  'why-rust',
  '## The Turning Point

After years of Excel macros, Power BI calculated columns, and the occasional Python script, I needed something that felt *precise*. Something where the compiler caught my mistakes before production did.

Rust fit that requirement exactly.

## What Changed

The borrow checker felt restrictive for the first two weeks. Then something clicked — it wasn''t restricting me, it was describing reality. Memory *does* have an owner. References *do* have lifetimes. The compiler was just making me be honest about it.

## Building With It

I now use Rust for:

- **Backend web**: Axum + Leptos for this site
- **Data tooling**: Delta Lake bindings and custom pipeline stages
- **CLI utilities**: Small tools that solve specific daily problems
- **Systems work**: NixOS configuration helpers

## The Trade-off

Build times. That''s it. Everything else is a win.',
  'After years of Power BI and Python, I needed something precise. The Rust compiler was that thing.',
  'Blog',
  '2026-02-15 10:00:00',
  '["rust","career","engineering"]'
),

(
  'Building a Data Pipeline in Rust',
  'data-pipeline-rust',
  '## The Problem

Most data pipeline tooling is Python-first. That means runtime errors, slow processing, and dependency hell. I wanted to explore what a Rust-native pipeline could look like.

## Architecture

Source (CSV / API) → Parser → Transform → Delta Lake sink

Each stage is a strongly-typed struct implementing a `Stage` trait. Errors are `Result` types all the way down — no panics in production paths.

## Delta Lake Integration

Delta Lake gives you ACID transactions on top of Parquet files. The Rust bindings (`delta-rs`) are still maturing but already usable for:

- Append-only writes
- Schema enforcement
- Time-travel queries

## What I Learned

Type-driven design at the pipeline level eliminates entire classes of bugs. When your schema is a Rust struct, a column rename is a compile error, not a midnight alert.',
  'Exploring what a type-safe, Rust-native data pipeline looks like using Delta Lake.',
  'Dev',
  '2026-02-10 08:30:00',
  '["rust","data","delta-lake","engineering"]'
),

(
  'NixOS: Reproducible Everything',
  'nixos-reproducible',
  '## What Is NixOS

NixOS is an operating system where the entire configuration lives in a single file (or a set of modules). Packages, services, users, dotfiles — all declared, all reproducible.

## Why It Matters for Remote Work

I work from Jamaica. Power cuts happen. When my machine needs to be rebuilt, I want it back to exactly the same state in under an hour. NixOS makes that possible.

```nix
{ pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    helix
    rust-analyzer
    nil
    starship
  ];
}
That''s it. That''s my editor setup. On any machine.

The Learning Curve
Nix the language is strange. It''s lazy, functional, and unforgiving. The error messages are improving but still cryptic. Worth it.',
'Why a reproducible OS matters when you work remotely and power cuts are a fact of life.',
'Dev',
'2026-01-28 11:00:00',
'["nix","nixos","infrastructure","linux"]'
),

(
'Graph Databases and Why They Click',
'graph-databases-neo4j',
'## Relational vs Graph

Relational databases ask: what are the rows?
Graph databases ask: what are the relationships?

For most data I work with — skills, people, projects, dependencies — the relationships are the data.

Neo4j in Practice

MATCH (p:Person)-[:SKILLED_IN]->(s:Skill)<-[:REQUIRES]-(j:Job)
WHERE s.name = "Rust"
RETURN p.name, j.title
That query would be three joins in SQL. In Cypher it reads almost like English.

Where I''m Using It
Modelling the relationship between my skills, projects, and learning goals. The graph makes it obvious which skills unlock the most opportunities.',
'Why graph databases model real-world relationships better than joins ever could.',
'Dev',
'2026-01-20 09:15:00',
'["neo4j","graph","data","databases"]'
),

(
'Solar Power for Remote Work in Jamaica',
'solar-remote-work-jamaica',
'## The Problem

JPS (Jamaica Public Service) cuts are unpredictable. A 2-hour outage in the middle of a client session is not acceptable. I needed a reliable power solution that didn''t depend on the grid.

The Setup
After research, I landed on a 400W panel + 1kWh LiFePO4 battery station. Enough to run:

Laptop (65W)

Router + modem (~20W)

Monitor (~30W)

Phone charging, lights

For roughly 8–10 hours between significant solar input.

What I''d Do Differently
Start with the battery capacity, not the panel wattage. The panel is cheap. The battery is the bottleneck.',
'How I built a solar backup system to keep working through Jamaica''s frequent power cuts.',
'Blog',
'2026-01-10 14:00:00',
'["jamaica","solar","remote-work","infrastructure"]'
);
