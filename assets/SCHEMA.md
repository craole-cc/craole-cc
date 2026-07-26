# Content schema

`assets/` is the Git-tracked source of truth for portfolio content and static media. `content` validates
these files before they are synced into SQLite or used by a future static snapshot.

Run commands from the repository root:

```bash
cargo run -p content -- validate .
cargo run -p content -- export-sql . | sqlite3 database/data/portfolio.db
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db
cargo run -p content -- export-json . dist/data
cargo run -p content -- export-static . dist
```

`validate` fails fast on malformed content. `export-sql` prints a deterministic SQLite seed script
that can hydrate the runtime database from the Git-tracked content files after migrations have
created the schema.

## Workflow

1. Add or edit files under `assets/projects/`, `assets/posts/`, or `assets/media/`.
2. Put local referenced image assets under `assets/media/images/`; HTTPS media URLs may be used for externally hosted royalty-free media.
3. Run `cargo run -p content -- validate .`.
4. Sync canonical images into the generated runtime delivery directory:

   ```bash
   ./scripts/sync-media-assets.sh
   ```

5. Rebuild the local database if needed:

   ```bash
   ./scripts/init-db.rs --reset
   cargo run -p content -- export-sql . | sqlite3 database/data/portfolio.db
   ```

6. Run the full quality gate before committing:

   ```bash
   bash scripts/ci.sh
   ```

## Draft templates

Use `content new` to create starter files:

```bash
cargo run -p content -- new project my-project
cargo run -p content -- new post project-build-log
cargo run -p content -- new media portfolio-screenshot
```

The command writes to the matching content directory:

- `project` → `assets/projects/<slug>.toml`
- `post` → `assets/posts/<slug>.md`
- `media` → `assets/media/<slug>.toml`

It rejects invalid slugs and refuses to overwrite existing files. Project and post templates are valid
unpublished drafts; media templates require the referenced asset to be added under `public/` before
validation passes.

## Database sync

Use `sync-db` to validate content, apply SQL migrations from `database/migrations/`, export the seed
SQL, apply it to SQLite, and print final counts:

```bash
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db
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
cargo run -p content -- export-json . dist/data
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
cargo run -p content -- export-static . dist
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
- Prefer relative, repository-owned media paths rooted in `public/` for work we control; use HTTPS URLs for royalty-free externally hosted media when avoiding local storage is important.
- Remote media is hotlinked at request time and is not downloaded, cached, or backed up by TheOracle. Keep attribution/source details in the caption or tags and expect external URLs to change or become unavailable.
- Treat `published = false` as draft mode: valid content may exist without being surfaced publicly.

## Projects

Project files live in `assets/projects/*.toml`.

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
- Screenshot paths use the flat image namespace, so `/media/images/demo_home.webp` maps to
  `assets/media/images/demo_home.webp`.

Recommended authoring checklist:

- Choose a slug that can remain stable after launch.
- Write `description` as a card summary, not a full README.
- Add tags that match the technologies visitors should notice.
- Include `repo_url` for public engineering work and `live_url` when there is a deployed artifact.
- Use `sort_order` to control featured ordering without renaming files.

## Posts

Post files live in `assets/posts/*.md` and use YAML-style or TOML-style frontmatter.

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

Media files live in `assets/media/*.toml`.

```toml
title = "Blue Mountain Study"
slug = "blue-mountain-study"
media_type = "photo"
file_path = "/media/images/blue-mountain-study_image.webp"
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
- `file_path` may be rooted in `public/` and point to an existing file, or may be an HTTPS URL for externally hosted media.
- Published media requires non-empty `alt_text`.
- `width` and `height`, when present, must be positive.
- `taken_at`, when present, must use `YYYY-MM-DD`.

Recommended authoring checklist:

- Store local image files flat in `assets/media/images/` with descriptive names such as
  `bass-plum-mint_avatar.svg` or `lms-analysis_web-dashboard.png`.
- Prefer web-friendly formats such as `.webp` for images and `.mp4` for video.
- Always write meaningful `alt_text`; it is required for published media.
- Keep large originals outside the repo unless they are intentionally part of the delivered site.
- Use tags for filtering and related-content discovery.

## `content` reference

```text
Usage: content <command> [args]
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
cargo run -p content -- validate .

# Validate another checkout.
cargo run -p content -- validate /path/to/craole-cc

# Export seed SQL to inspect before applying it.
cargo run -p content -- export-sql . > /tmp/craole-content.sql

# Create a new draft project template.
cargo run -p content -- new project my-project

# Validate, migrate, and seed the local SQLite database.
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db

# Export static fallback JSON data.
cargo run -p content -- export-json . dist/data

# Export minimal static fallback HTML and JSON.
cargo run -p content -- export-static . dist

# Apply exported content to the local database.
cargo run -p content -- export-sql . | sqlite3 database/data/portfolio.db
```
