# Changelog

All notable changes to Craole.CC are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Fixed

## [0.2.14] - 2026-07-26

### Added

- Allow Art media to use HTTPS URLs for externally hosted royalty-free images, including Unsplash-style image hosts, without storing the binary on TheOracle.
- Add validation coverage for accepted HTTPS media URLs and rejected insecure HTTP URLs.

### Changed

- Keep local `public/` media and remote HTTPS media as supported alternatives through the same `file_path` field.
- Document the storage, hotlinking, attribution, and link-rot trade-offs of remote media.

## [0.2.13] - 2026-07-26

### Added

- Added four locally authored bass artwork studies to the Art catalogue with captions, dimensions, tags, accessible alt text, and public PNG assets.

### Changed

- Consolidated the former Data view into the Dev project catalogue.
- Kept `/data` as a compatibility route that renders the unified Dev catalogue instead of maintaining a second page.
- Removed the stale Data page from the static exporter and sitemap.

### Fixed

- Restored the Art page's media records and assets after the media table was empty and no `content/media` source files were present.
- Preserved all published project records in the Dev query; the catalogue currently contains the two project records defined by source content.

## [0.2.8] - 2026-07-25

### Fixed

- Added an explicit stylesheet version query so browsers fetch the updated footer icon colors instead of retaining cached CSS.

## [0.2.7] - 2026-07-25

### Fixed

- Ensured the footer's normal social icons use the primary text color despite brand-specific icon classes; hover colors remain unchanged.

## [0.2.6] - 2026-07-25

### Changed

- Changed the footer's default social icons from muted color to the primary text color while preserving the existing hover icons.

## [0.2.5] - 2026-07-25

### Changed

- Replaced the homepage Vision statement with the updated craft, musicianship, and software/data systems wording.

## [0.2.4] - 2026-07-25

### Fixed

- Prevented the Log page's duplicate post resources from overwriting populated results with an empty client-side state after hydration.

## [0.2.3] - 2026-07-24

### Changed

- Served read-only public pages from the generated static export, retaining the Rust application as the dynamic fallback.
- Reduced first-paint hero work from fourteen external 1920px background images to one responsive 1200px image.
- Removed the hero slideshow timer and repeated client-side image/hue processing.
- Added compressed delivery and browser caching for static assets at the reverse-proxy layer.

## [0.2.2] - 2026-07-24

### Changed

- Replaced placeholder and aspirational profile copy with an author-provided account of Craig Cole's music, creative, technical, BPO, L&D, BI, TEFL, and software journey.
- Added the long-term music affiliations Skygrass (formerly Blu Grass in the Sky), No-maddz, Stone Dub, Protoje & The Indiggnation, and BLACK as COLE.
- Added the public profile/CV post and expanded the website About and Professional Background sections with verified career evidence.
- Clarified the connection between music, creative production, deep-focus work, asynchronous collaboration, and Rust-first software development.
- Updated the Fluentbe and independent-freelance teaching history, including the May 2022 Fluentbe start and reduced current freelance pace.

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

[Unreleased]: https://github.com/craole-cc/craole-cc/compare/v0.2.14...HEAD
[0.2.14]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.14
[0.2.7]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.7
[0.2.6]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.6
[0.2.5]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.5
[0.2.4]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.4
[0.2.3]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.3
[0.2.2]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.2
[0.2.1]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.1
[0.2.0]: https://github.com/craole-cc/craole-cc/releases/tag/v0.2.0
