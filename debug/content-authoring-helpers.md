# Content authoring helpers

## Summary

This phase adds `contentctl new` so portfolio content starts from consistent draft templates instead
of hand-written files.

## Commands added

```sh
cargo run -p contentctl -- new project <slug>
cargo run -p contentctl -- new post <slug>
cargo run -p contentctl -- new media <slug>
```

Optional final argument:

```sh
cargo run -p contentctl -- new project <slug> /path/to/repo-root
```

## Behavior

- Rejects invalid slugs using the same lowercase/hyphenated slug rule as validation.
- Creates parent directories as needed.
- Refuses to overwrite an existing content file.
- Writes project templates to `content/projects/<slug>.toml`.
- Writes post templates to `content/posts/<slug>.md`.
- Writes media templates to `content/media/<slug>.toml`.

Project and post templates are valid unpublished drafts immediately. Media templates still need the
referenced asset added under `public/` before `contentctl validate` can pass.

## Verification

Focused TDD loop:

```text
cargo test -p contentctl --test new_content
```

Full local quality gate:

```text
nix develop --command bash scripts/ci.sh
```

## Next possible phase

A stronger database sync command can wrap the current export path:

```sh
cargo run -p contentctl -- sync-db . sqlite://database/data/portfolio.db
```

That command should validate content, ensure migrations have run, apply the exported seed SQL, and
print seeded counts.
