<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to Craole.CC are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.21] - 2026-07-27

### Fixed

- Make Git-tracked TOML and Markdown the production content source of truth while retaining SQLite as the migrated runtime index.
- Preserve the eight canonical projects that previously existed only in the development database.
- Run content synchronization through SQLx migration tracking instead of replaying raw migration files.
- Back up, stage, synchronize, switch, and roll back the production database together with each versioned release.
- Restore the original checksum of applied migration `0005`.

## [0.2.20] - 2026-07-27

### Fixed

- Restore the original checksum of applied migration `0003` so production SQLx migrations remain compatible with the live database.
## [0.2.19] - 2026-07-27

### Fixed

- Track canonical PNG assets under `assets/` so clean CI checkouts and production deployments include the Art and LMS images.
- Remove the overly broad image ignore rules that caused public assets to remain local-only.

## [0.2.18] - 2026-07-26

### Changed

- Update `pulldown-cmark`, `wasm-bindgen`, `wasm-bindgen-futures`, and `web-sys` to current compatible releases.
- Update the pinned Nixpkgs input and matching `wasm-bindgen` CLI for reproducible release builds.
- Remove tracked temporary diagnostics and normalize formatting after the dependency update.

## [0.2.17] - 2026-07-26

### Changed

- Consolidated static assets into `assets/` and removed `public/`.
- Moved private content to `content/assets/` to protect TOML/Markdown files.
- Introduced project media bundles with auto-expansion and audio support.
- Normalized the LMS slug to `lms-analysis` and bundled its assets.

## [0.2.16] - 2026-07-26

### Changed

- Flattened local image assets under `assets/media/images/`.
- Removed redundant avatar, art, and project image subdirectories.
- Served `assets/` directly without generating a `public/` copy.
- Isolated private metadata inside `content/assets/`, outside asset root.

## [0.2.15] - 2026-07-26

### Changed

- Reorganized portfolio sources, moving material from `content/` to `assets/`.
- Renamed the content CLI crate from `contentctl` to `content`.
- Structured workspace so `assets/` holds raw data and `content/` holds code.
- Maintained existing validation, SQLite seed, export, and media workflows.

## [0.2.14] - 2026-07-26

### Added

- Support for external HTTPS URLs to avoid storing local binaries.
- Validation rules to enforce HTTPS and reject insecure HTTP links.

### Changed

- Supported both local `assets/` and remote HTTPS media via `file_path`.
- Documented storage, hotlinking, attribution, and link-rot trade-offs.

## [0.2.13] - 2026-07-26

### Added

- Four bass artwork studies with metadata, alt text, and export files.

### Changed

- Merged the Data page into the Dev project catalogue.
- Redirected `/data` to render the unified Dev catalogue.
- Removed the obsolete Data page from the sitemap and static exporter.

### Fixed

- Restored Art page media records and assets after data loss.
- Preserved all published project records in the Dev query.

## [0.2.8] - 2026-07-25

### Fixed

- Versioned stylesheet queries to bypass cached footer icon styles.

## [0.2.7] - 2026-07-25

### Fixed

- Set footer social icons to text colour while keeping hover styles intact.

## [0.2.6] - 2026-07-25

### Changed

- Updated default footer social icons from muted to primary text colour.

## [0.2.5] - 2026-07-25

### Changed

- Updated homepage Vision to focus on craft, music, and software systems.

## [0.2.4] - 2026-07-25

### Fixed

- Prevented hydration from overwriting Log posts with an empty state.

## [0.2.3] - 2026-07-24

### Changed

- Served read-only pages via static export with Axum dynamic fallback.
- Replaced hero slideshow and heavy backgrounds with one 1200px image.
- Enabled compression and browser caching for static assets at proxy layer.

## [0.2.2] - 2026-07-24

### Changed

- Updated profile copy on Craig Cole's music and software background.
- Included affiliations: Skygrass, No-Maddz, Stone Dub, Protoje, BLACK as COLE.
- Expanded About, Background, and CV entries with verified career history.
- Clarified links between music production, async work, and Rust dev.
- Updated teaching history details for Fluentbe and freelance work.

## [0.2.1] - 2026-07-24

### Changed

- Removed obsolete deployment attribution from the footer.
- Stripped incremental/debug build storage to trim CI disk usage.
- Dropped Magic Nix Cache and duplicate branch triggers from CI.

### Fixed

- Defaulted CI target directory selection to the local Cargo path.

## [0.2.0] - 2026-07-24

### Added

- Local-first content pipeline for projects, posts, and media.
- Dedicated `content` CLI for validation, drafts, exports, and static builds.
- Data page, structured projects, and bass avatar variants.
- GitHub Actions workflows for CI and Pages deployment.
- Developer diagnostics and content pipeline documentation.

### Changed

- Migrated portfolio to Leptos/Axum/SQLite with Markdown/TOML sources.
- Aligned static page shell with the main app interface.
- Restored local Leptos dev build workflow.
- Pointed project metadata to `craole-cc/craole-cc`.

### Fixed

- Fixed Data page render and removed unintended branding.
- Improved database sync and static export reliability.
- Test coverage for validation, export, and database sync in CI.

[Unreleased]: https://github.com/craole-cc/craole-cc/compare/v0.2.21...HEAD
[0.2.21]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.21
[0.2.20]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.20
[0.2.19]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.19
[0.2.18]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.18
[0.2.17]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.17
[0.2.16]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.16
[0.2.15]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.15
[0.2.14]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.14
[0.2.13]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.13
[0.2.8]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.8
[0.2.7]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.7
[0.2.6]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.6
[0.2.5]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.5
[0.2.4]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.4
[0.2.3]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.3
[0.2.2]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.2
[0.2.1]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.1
[0.2.0]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.0
