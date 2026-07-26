# Static site export

## Summary

This phase adds `content export-static`, the first end-to-end static fallback generator. It turns
validated Git-tracked content into plain HTML route files plus the existing static JSON data export.

## Command

```sh
cargo run -p content -- export-static . dist
```

If the output directory is omitted, it defaults to `dist`.

## Output

```text
dist/index.html
dist/dev/index.html
dist/dev/<project-slug>/index.html
dist/log/index.html
dist/log/<post-slug>/index.html
dist/art/index.html
dist/404.html
dist/sitemap.xml
dist/data/projects.json
dist/data/posts.json
dist/data/media.json
dist/data/manifest.json
```

The exporter also copies files from ``assets/` into the output directory so referenced assets are
available to the static fallback.

## Behavior

- validates content before creating output
- renders only published projects/posts/media into public HTML pages
- still exports JSON data through the existing `export-json` path
- generates a dark, simple, self-contained HTML/CSS fallback
- renders basic Markdown headings and paragraphs for posts
- writes a `404.html` and `sitemap.xml`

Expected success output:

```text
static site exported to dist: pages=7 projects=1 posts=1 media=0
```

## Verification

Focused TDD loop:

```text
cargo test -p content --test export_static
```

Full quality gate now smoke-tests both static JSON and static HTML export:

```text
nix develop --command bash scripts/ci.sh
```

## Known limitations

- Markdown rendering is intentionally minimal for now: headings and paragraphs only.
- The generated visual design is a fallback/preview, not a replacement for the full Leptos SSR UI.
- Sitemap URLs are relative until a canonical site URL is added to configuration.

## Next possible phase

Improve static rendering fidelity by sharing design tokens or adding a small Markdown renderer, then
keep the static export documented as a local preview and rollback artifact for the existing
server-backed deployment.
