# Content schema

`contentctl validate` checks Git-tracked content before it is synced into SQLite or used by a static snapshot.

Run it from the repository root:

```bash
cargo run -p contentctl -- validate .
cargo run -p contentctl -- export-sql . | sqlite3 database/data/portfolio.db
```

`validate` fails fast on malformed content. `export-sql` prints a SQLite seed script that can hydrate the runtime database from the Git-tracked content files after migrations have created the schema.

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
- Screenshot paths are rooted in `public/`, so `/media/projects/demo/home.webp` maps to `public/media/projects/demo/home.webp`.

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
