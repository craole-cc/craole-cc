# Content schema

`content/` is the Git-tracked source of truth for portfolio content. `contentctl` validates these
files before they are synced into SQLite or used by a future static snapshot.

Run commands from the repository root:

```bash
cargo run -p contentctl -- validate .
cargo run -p contentctl -- export-sql . | sqlite3 database/data/portfolio.db
cargo run -p contentctl -- sync-db . sqlite://database/data/portfolio.db
cargo run -p contentctl -- export-json . dist/data
cargo run -p contentctl -- export-static . dist
```

`validate` fails fast on malformed content. `export-sql` prints a deterministic SQLite seed script
that can hydrate the runtime database from the Git-tracked content files after migrations have
created the schema.

## Workflow

1. Add or edit files under `content/projects/`, `content/posts/`, or `content/media/`.
2. Put referenced binary/static assets under `public/`.
3. Run `cargo run -p contentctl -- validate .`.
4. Rebuild the local database if needed:

   ```bash
   ./scripts/init-db.rs --reset
   cargo run -p contentctl -- export-sql . | sqlite3 database/data/portfolio.db
   ```

5. Run the full quality gate before committing:

   ```bash
   bash scripts/ci.sh
   ```

## Draft templates

Use `contentctl new` to create starter files:

```bash
cargo run -p contentctl -- new project my-project
cargo run -p contentctl -- new post project-build-log
cargo run -p contentctl -- new media portfolio-screenshot
```

The command writes to the matching content directory:

- `project` → `content/projects/<slug>.toml`
- `post` → `content/posts/<slug>.md`
- `media` → `content/media/<slug>.toml`

It rejects invalid slugs and refuses to overwrite existing files. Project and post templates are valid
unpublished drafts; media templates require the referenced asset to be added under `public/` before
validation passes.

## Database sync

Use `sync-db` to validate content, apply SQL migrations from `database/migrations/`, export the seed
SQL, apply it to SQLite, and print final counts:

```bash
cargo run -p contentctl -- sync-db . sqlite://database/data/portfolio.db
```

If the database URL argument is omitted, the command uses `DATABASE_URL`. If `DATABASE_URL` is not
set, it falls back to `sqlite://database/data/portfolio.db`.

Expected output:

```text
content database synced: projects=1 posts=1 media=0
```

## Static JSON export

Use `export-json` to produce data files for static fallback work:

```bash
cargo run -p contentctl -- export-json . dist/data
```

It writes `projects.json`, `posts.json`, `media.json`, and `manifest.json`. The manifest contains
item counts so build scripts can smoke-test the export without parsing every file.

Expected output:

```text
static JSON exported to dist/data: projects=1 posts=1 media=0
```

## Static HTML export

Use `export-static` to produce a minimal static fallback site:

```bash
cargo run -p contentctl -- export-static . dist
```

It writes HTML route files for the home page, project index, project detail pages, log index, post
detail pages, art index, `404.html`, and `sitemap.xml`. It also writes `dist/data/*.json` by running
the same JSON export path.

Expected output:

```text
static site exported to dist: pages=7 projects=1 posts=1 media=0
```

## General rules

- Slugs are stable public identifiers. Prefer lowercase ASCII letters, numbers, and hyphens.
- Keep titles human-readable and portfolio-ready.
- Keep descriptions concise enough for cards, indexes, and search results.
- Prefer relative, repository-owned media paths rooted in `public/`.
- Treat `published = false` as draft mode: valid content may exist without being surfaced publicly.

## Projects

Project files live in `content/projects/*.toml`.

```toml
title = "Demo Project"
slug = "demo-project"
status = "building"
description = "Short, portfolio-ready project summary."
repo_url = "https://github.com/craole-cc/demo-project"
live_url = "https://example.com"
featured = true
published = true
sort_order = 10
tags = ["Rust", "Leptos"]
screenshots = ["/media/projects/demo/home.webp"]
```

Rules:

- `title`, `slug`, `status`, and `description` are required.
- `slug` must be unique and use lowercase ASCII letters, numbers, and hyphens.
- `status` must be one of: `active`, `building`, `planning`, `archived`.
- `repo_url` and `live_url`, when present, must be HTTP(S) URLs.
- `sort_order`, when present, must be non-negative.
- `featured = true` requires `published = true`.
- Screenshot paths are rooted in `public/`, so `/media/projects/demo/home.webp` maps to
  `public/media/projects/demo/home.webp`.

Recommended authoring checklist:

- Choose a slug that can remain stable after launch.
- Write `description` as a card summary, not a full README.
- Add tags that match the technologies visitors should notice.
- Include `repo_url` for public engineering work and `live_url` when there is a deployed artifact.
- Use `sort_order` to control featured ordering without renaming files.

## Posts

Post files live in `content/posts/*.md` and use YAML-style or TOML-style frontmatter.

```markdown
---
title: "Hello, Craole.CC"
slug: "hello-craole-cc"
kind: "blog"
published: true
published_at: "2026-06-03"
tags: ["Rust", "Leptos"]
excerpt: "First post in the portfolio."
---

# Hello

Body content goes here.
```

Rules:

- `title`, `slug`, and `kind` are required.
- `slug` must be unique and use lowercase ASCII letters, numbers, and hyphens.
- `kind` must be one of: `blog`, `cv`, `note`.
- `published_at`, when present, must use `YYYY-MM-DD`.
- Published posts must have a non-empty body.
- Missing tags currently produce a warning, not a failure.

Recommended authoring checklist:

- Use `kind = "blog"` for public essays, `kind = "note"` for shorter logs, and `kind = "cv"` for
  résumé/CV content.
- Include `excerpt` for cards, archive pages, and search snippets.
- Keep the first heading aligned with the title unless there is a deliberate editorial reason not to.
- Validate before committing so frontmatter errors are caught outside the app runtime.

## Media

Media files live in `content/media/*.toml`.

```toml
title = "Blue Mountain Study"
slug = "blue-mountain-study"
media_type = "photo"
file_path = "/media/art/blue-mountain-study.webp"
alt_text = "Abstract blue mountain landscape."
published = true
sort_order = 10
taken_at = "2026-06-03"
width = 1600
height = 900
tags = ["art", "portfolio"]
```

Rules:

- `title`, `slug`, `media_type`, and `file_path` are required.
- `slug` must be unique and use lowercase ASCII letters, numbers, and hyphens.
- `media_type` must be one of: `photo`, `video`.
- `file_path` is rooted in `public/` and must point to an existing file.
- Published media requires non-empty `alt_text`.
- `width` and `height`, when present, must be positive.
- `taken_at`, when present, must use `YYYY-MM-DD`.

Recommended authoring checklist:

- Prefer web-friendly formats such as `.webp` for images and `.mp4` for video.
- Always write meaningful `alt_text`; it is required for published media.
- Keep large originals outside the repo unless they are intentionally part of the delivered site.
- Use tags for filtering and related-content discovery.

## `contentctl` reference

```text
Usage: contentctl <command> [args]
Commands:
  validate [repo-root]              Validate content files
  export-sql [repo-root]            Print a SQLite seed script generated from content files
  export-json [repo-root] [output-dir]
                                  Export static JSON data files
  export-static [repo-root] [output-dir]
                                  Export static fallback HTML and JSON
  new <project|post|media> <slug> [repo-root]
                                  Create a draft content template
  sync-db [repo-root] [database-url]
                                  Apply migrations and sync content into SQLite
```

Examples:

```bash
# Validate the current repository.
cargo run -p contentctl -- validate .

# Validate another checkout.
cargo run -p contentctl -- validate /path/to/craole-cc

# Export seed SQL to inspect before applying it.
cargo run -p contentctl -- export-sql . > /tmp/craole-content.sql

# Create a new draft project template.
cargo run -p contentctl -- new project my-project

# Validate, migrate, and seed the local SQLite database.
cargo run -p contentctl -- sync-db . sqlite://database/data/portfolio.db

# Export static fallback JSON data.
cargo run -p contentctl -- export-json . dist/data

# Export minimal static fallback HTML and JSON.
cargo run -p contentctl -- export-static . dist

# Apply exported content to the local database.
cargo run -p contentctl -- export-sql . | sqlite3 database/data/portfolio.db
```
