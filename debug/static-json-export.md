# Static JSON export

## Summary

This phase adds `content export-json` as the first concrete static-fallback artifact. It turns
validated Git-tracked content into JSON files that a static site, prerenderer, or client-side
fallback can consume without a live SQLite database.

## Command

```sh
cargo run -p content -- export-json . dist/data
```

If the output directory is omitted, it defaults to `dist/data`.

## Output files

```text
dist/data/projects.json
dist/data/posts.json
dist/data/media.json
dist/data/manifest.json
```

`manifest.json` contains counts for projects, posts, and media so build scripts can smoke-test the
export cheaply.

## Behavior

- validates content before writing anything
- refuses to export invalid content
- preserves post Markdown bodies in the JSON payload
- writes pretty JSON with a trailing newline for stable diffs

Expected success output:

```text
static JSON exported to dist/data: projects=1 posts=1 media=0
```

## Verification

Focused TDD loop:

```text
cargo test -p content --test export_json
```

Full local quality gate:

```text
nix develop --command bash scripts/ci.sh
```

## Next possible phase

A minimal static fallback can now consume `dist/data/*.json` and render either:

- a small standalone static HTML page, or
- a future `static-export` command that writes route-specific HTML under `dist/`.
