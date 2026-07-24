# Changelog

All notable changes to Craole.CC are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.1] - 2026-07-24

### Changed

- Removed the obsolete Oxyde.Cloud deployment attribution from the footer.
- Reduced GitHub Actions runner disk usage by disabling incremental and debug
  artifact storage for CI builds.
- Removed deprecated Magic Nix Cache integration and duplicate branch/PR CI
  triggers.

### Fixed

- Made the CI target-directory selection independent of Python by using the
  repository-local Cargo target directory by default.

## [0.2.0] - 2026-07-24

### Added

- Added a local-first content pipeline for projects, posts, and media.
- Added the `contentctl` Rust CLI for validating content, generating draft
  templates, exporting JSON, exporting SQL seed data, synchronizing SQLite,
  and producing a static fallback site.
- Added the Data page and structured project content.
- Added bass-themed avatar artwork and a workshop of visual variants.
- Added GitHub Actions CI and Pages deployment workflows.
- Added developer diagnostics for port inspection and guarded Leptos watch
  workflows.
- Added content-pipeline documentation, validation tests, and static-export
  smoke tests.

### Changed

- Moved the portfolio toward a Rust-first Leptos/Axum/SQLite architecture with
  a tracked Markdown/TOML content source of truth.
- Improved the portfolio shell and aligned static pages with the application
  experience.
- Restored the end-to-end Leptos development build and documented the required
  local workflow.
- Updated the project metadata and repository links for the canonical
  `craole-cc/craole-cc` repository.

### Fixed

- Repaired the Data page and removed unintended site branding.
- Improved content database synchronization and static export behavior.
- Added CI coverage for content validation, export paths, and database sync.

[Unreleased]: https://github.com/craole-cc/craole-cc/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.1
[0.2.0]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.0
