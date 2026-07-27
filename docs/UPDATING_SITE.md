# Updating Craole.CC

This is the owner runbook for adding or changing projects, logs/blog posts, and media on
[craole.cc](https://craole.cc).

## The important rule

Edit the Git-tracked source files—not SQLite and not production files:

```text
content/assets/posts/*.md                 blog, note, and profile content
content/assets/projects/*.toml            project catalogue
content/assets/media/*.toml               standalone art/media metadata
content/assets/media/projects/*/config.toml
                                             project media metadata
assets/media/images/*                     local standalone images
assets/media/projects/*                   local project images/audio/video
```

`content/assets/` is private authored metadata and prose. Top-level `assets/` contains files the
website may serve. Never put private/raw data in `assets/`.

A push to `main` runs CI, builds the site, exports the fallback, backs up and synchronizes the
production database, switches the versioned release, and checks the public site. Do not edit
`/var/lib/craole-cc/portfolio.db` manually.

## Standard publishing workflow

Run commands from the repository root:

```sh
cd /home/craole-cc/Projects/cole-bassed/craole.cc
git pull --ff-only
nix develop
```

After adding or editing content:

```sh
# Validate authored Markdown/TOML and referenced local files.
cargo run -p content -- validate .

# Update the local development database.
cargo run -p content -- sync-db . sqlite://database/data/portfolio.db

# Preview the real application (3005 avoids the production backend on 3000).
leptoswatch 3005
```

Open these pages while previewing:

- `http://127.0.0.1:3005/dev` for projects
- `http://127.0.0.1:3005/log` for blogs and notes
- `http://127.0.0.1:3005/art` for published media

Before publishing, while still inside `nix develop`:

```sh
bash scripts/ci.sh
git diff --check
git status --short
git diff
```

From outside the development shell, the equivalent one-shot quality gate is
`nix develop --command bash scripts/ci.sh`.

Commit only reviewed content and public assets:

```sh
git add content/assets assets CHANGELOG.md
git commit -m "content: add <short description>"
git push origin main
```

Pushing `main` automatically deploys after CI succeeds. Follow it with:

```sh
gh run list --branch main --limit 1
gh run watch <run-id> --exit-status
```

Then verify the affected public page and any local assets:

```sh
curl --fail https://craole.cc/dev >/dev/null
curl --fail https://craole.cc/log >/dev/null
curl --fail https://craole.cc/art >/dev/null
curl --fail https://craole.cc/media/path/to/file.webp >/dev/null
```

A version bump and Git tag are optional for routine content updates. Use them when the change should
be a named release; deployment itself is triggered by the push to `main`.

## Add a blog or log entry

Create a safe unpublished template:

```sh
cargo run -p content -- new post my-post-slug
```

Edit `content/assets/posts/my-post-slug.md`:

```markdown
---
title: "Human-readable title"
slug: "my-post-slug"
kind: "blog"
published: false
published_at: "2026-07-27"
cover_url: "https://images.unsplash.com/photo-example?auto=format&fit=crop&w=1200&q=80"
tags: ["rust", "data"]
excerpt: "A short summary used on the log index and in search results."
---

# Human-readable title

Write the complete Markdown body here.
```

Use:

- `kind: "blog"` for articles;
- `kind: "note"` for shorter logs;
- `kind: "cv"` for the profile/CV entry.

Keep `published: false` while drafting. Set it to `true` only when the body, excerpt, date, links,
and cover are ready. Published posts require a non-empty body. The filename and `slug` must match.

For a local cover, put the image under top-level `assets/` and use its public path, for example:

```yaml
cover_url: "/media/images/my-post-cover.webp"
```

## Add a project

Create a template:

```sh
cargo run -p content -- new project my-project
```

Edit `content/assets/projects/my-project.toml`:

```toml
title = "My Project"
slug = "my-project"
status = "building"
description = "A concise, truthful summary suitable for the project card."
repo_url = "https://github.com/craole-cc/my-project"
live_url = "https://example.com"
featured = false
published = false
sort_order = 50
tags = ["Rust", "Leptos"]
screenshots = ["/media/projects/my-project/images/dashboard.webp"]
```

Allowed statuses are `active`, `building`, `planning`, and `archived`.

If screenshots are listed, add the actual files at the matching served location:

```text
assets/media/projects/my-project/images/dashboard.webp
```

The validator fails if a referenced local file is missing. Set `published = true` when ready.
`featured = true` is only valid for a published project. Use `sort_order` instead of renaming files
to control ordering.

Do not copy another project's source into the portfolio. Link to its canonical repository and live
application.

## Add standalone art or media

Create a template:

```sh
cargo run -p content -- new media my-art-slug
```

### Local image

Put the image at a public path such as:

```text
assets/media/images/my-art-slug.webp
```

Then edit `content/assets/media/my-art-slug.toml`:

```toml
title = "My Art Title"
slug = "my-art-slug"
caption = "A truthful description or attribution."
media_type = "photo"
file_path = "/media/images/my-art-slug.webp"
alt_text = "A useful visual description for someone who cannot see the image."
published = false
sort_order = 130
taken_at = "2026-07-27"
width = 1600
height = 900
tags = ["art", "illustration"]
```

The schema currently uses `media_type = "photo"` for still images, including illustrations. Describe
illustrations as illustrations in the title, caption, alt text, or tags; do not call them photographs.

### Unsplash/external image

Keep the image external and use the direct HTTPS CDN URL:

```toml
title = "Blue Mountain Study"
slug = "blue-mountain-study"
caption = "Photo by Photographer Name on Unsplash."
media_type = "photo"
file_path = "https://images.unsplash.com/photo-example?auto=format&fit=crop&w=1920&q=80"
alt_text = "Blue mountains beneath a pale sky."
published = true
sort_order = 140
width = 1920
height = 1280
tags = ["art", "photography", "unsplash"]
```

Use `https://images.unsplash.com/...`, not an `https://unsplash.com/photos/...` webpage. Preserve
photographer/source attribution. External media is hotlinked and is not included in server backups,
so use local files for irreplaceable work.

Before publishing an external image, check it:

```sh
curl -L --fail --output /dev/null \
  "https://images.unsplash.com/photo-example?auto=format&fit=crop&w=1920&q=80"
```

## Add project-specific media

Store served files beneath the canonical project directory:

```text
assets/media/projects/my-project/images/
assets/media/projects/my-project/audio/
assets/media/projects/my-project/video/
```

Create `content/assets/media/projects/my-project/config.toml`:

```toml
project_slug = "my-project"

[[assets]]
title = "My Project dashboard"
slug = "my-project-dashboard"
caption = "Synthetic demonstration dashboard."
media_type = "photo"
file_path = "/media/projects/my-project/images/dashboard.webp"
alt_text = "Dashboard showing synthetic metrics and filters."
published = false
sort_order = 10
width = 1600
height = 1000
tags = ["dashboard", "synthetic-data"]
```

Repeat `[[assets]]` for additional images, audio, or video. Use `media_type = "audio"` or
`media_type = "video"` where appropriate. Never place private source data, unsanitized exports, API
keys, or credentials in the served asset tree.

## Edit or remove existing content

- Edit the canonical file under `content/assets/` and keep its slug stable when possible.
- To hide an item without deleting its history, set `published = false` and deploy.
- To remove it entirely, delete its authored file and any unneeded served asset, validate, sync the
  local database, preview all affected pages, and deploy.
- Do not add ordinary content by changing `database/migrations/*.sql`. Applied migrations are
  checksum-sensitive, including comments.

## What deployment does

The production workflow:

1. validates content and runs the Rust/SQLx test suite;
2. builds the Leptos release and static fallback;
3. creates a timestamped copy of `/var/lib/craole-cc/portfolio.db`;
4. applies migrations and synchronizes authored content into a staged database;
5. atomically switches the database and versioned `/opt/craole-cc/releases/...` release;
6. restarts `craole-cc.service` and verifies local and public HTTP endpoints;
7. restores the previous database and release if health checks fail.

A successful Git push is not enough by itself: wait for the GitHub Actions deployment to finish and
check the public route.

## Common mistakes

- Editing SQLite or seed migrations instead of `content/assets/`.
- Forgetting `published = true` when expecting an item publicly.
- Forgetting to run `sync-db` before local preview.
- Putting authored/private TOML or Markdown under top-level `assets/`.
- Referencing a local file that is ignored by Git or was never committed.
- Using an Unsplash webpage URL instead of the direct image URL.
- Downloading external Unsplash images when they are intended to remain hotlinked.
- Exposing private/raw analytics data in a public asset or release.
- Claiming deployment success before CI and public route checks finish.

For every field and validation rule, see [the content schema](../content/SCHEMA.md). For development
and SQLx details, see [CONTRIBUTING.md](../CONTRIBUTING.md).
