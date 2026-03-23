-- ═══════════════════════════════════════════════════════════════════════════════
-- migrations/0012_seed_projects.sql — Seed Dev page with real projects
-- ═══════════════════════════════════════════════════════════════════════════════

-- ── Projects ──────────────────────────────────────────────────────────────────

INSERT INTO projects (title, description, status, repo_url, live_url, featured, published, sort_order) VALUES
(
  'craole.cc',
  'Full-stack portfolio and blog built entirely in Rust. Leptos handles reactive SSR with hydration, Axum serves the backend, SQLite stores content, and Tailwind + SCSS style every pixel. The site you''re looking at right now.',
  'active',
  'https://github.com/craole-cc/craole-cc',
  'https://craole.cc',
  1, 1, 10
),
(
  'wallter',
  'Dynamic desktop wallpaper manager that rotates backgrounds on a schedule. Cross-platform CLI built in Rust with a focus on clean argument parsing and OS-native integrations for Linux, macOS, and Windows.',
  'active',
  'https://github.com/craole-cc/wallter',
  NULL,
  1, 1, 20
),
(
  'datum',
  'Unified data engineering lab for building pipelines in both Rust and Python. Explores Delta Lake for versioned storage, medallion architecture (bronze/silver/gold), and practical patterns for ingestion, transformation, and analytics.',
  'building',
  'https://github.com/craole-cc/datum',
  NULL,
  1, 1, 30
),
(
  'ephelog',
  'Ephemeral logger that wraps the tracing ecosystem with sane defaults. Zero-config startup for CLI tools and servers — structured output, level filtering, and coloured terminal formatting out of the box.',
  'building',
  'https://github.com/craole-cc/ephelog',
  NULL,
  1, 1, 40
),
(
  'dotDots',
  'Declarative system configuration for NixOS, GNOME, and development environments. Manages shell configs, editor setups, and reproducible toolchains across multiple machines — infrastructure-as-code for the desktop.',
  'active',
  'https://github.com/craole-cc/dotDots',
  NULL,
  0, 1, 50
),
(
  'ccutils',
  'Collection of cross-platform shell utilities and developer productivity scripts. POSIX-compatible wrappers, environment helpers, and automation tools that smooth out day-to-day terminal workflows.',
  'active',
  'https://github.com/craole-cc/ccutils',
  NULL,
  0, 1, 60
),
(
  'gitsy',
  'Git repository initialiser with opinionated defaults. Sets up commit conventions, branch structure, and remote configuration in a single command — removing friction from starting new projects.',
  'planning',
  'https://github.com/craole-cc/gitsy',
  NULL,
  0, 1, 70
),
(
  'Advent of Code',
  'Solutions to Advent of Code challenges, implemented in Rust with a focus on idiomatic patterns, performance, and readable problem decomposition.',
  'active',
  'https://github.com/craole-cc/Advent-of-Code',
  NULL,
  0, 1, 80
),
(
  'say_data',
  'Exploration of data storytelling and analytics communication. Building tools and techniques for presenting data insights clearly — because the pipeline is only half the story.',
  'planning',
  'https://github.com/craole-cc/say_data',
  NULL,
  0, 1, 90
);

-- ── Tags ──────────────────────────────────────────────────────────────────────

-- craole.cc
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'Leptos'),
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'Axum'),
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'SQLite'),
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'Tailwind'),
  ((SELECT id FROM projects WHERE title = 'craole.cc'), 'Full-Stack');

-- wallter
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'wallter'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'wallter'), 'CLI'),
  ((SELECT id FROM projects WHERE title = 'wallter'), 'Cross-Platform'),
  ((SELECT id FROM projects WHERE title = 'wallter'), 'Desktop');

-- datum
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'datum'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'datum'), 'Python'),
  ((SELECT id FROM projects WHERE title = 'datum'), 'Delta Lake'),
  ((SELECT id FROM projects WHERE title = 'datum'), 'Data Engineering');

-- ephelog
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'ephelog'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'ephelog'), 'Tracing'),
  ((SELECT id FROM projects WHERE title = 'ephelog'), 'CLI'),
  ((SELECT id FROM projects WHERE title = 'ephelog'), 'Open Source');

-- dotDots
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'dotDots'), 'Nix'),
  ((SELECT id FROM projects WHERE title = 'dotDots'), 'NixOS'),
  ((SELECT id FROM projects WHERE title = 'dotDots'), 'Shell'),
  ((SELECT id FROM projects WHERE title = 'dotDots'), 'Infrastructure');

-- ccutils
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'ccutils'), 'Shell'),
  ((SELECT id FROM projects WHERE title = 'ccutils'), 'POSIX'),
  ((SELECT id FROM projects WHERE title = 'ccutils'), 'CLI'),
  ((SELECT id FROM projects WHERE title = 'ccutils'), 'Cross-Platform');

-- gitsy
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'gitsy'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'gitsy'), 'Git'),
  ((SELECT id FROM projects WHERE title = 'gitsy'), 'CLI');

-- Advent of Code
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'Advent of Code'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'Advent of Code'), 'Algorithms');

-- say_data
INSERT INTO project_tags (project_id, tag) VALUES
  ((SELECT id FROM projects WHERE title = 'say_data'), 'Rust'),
  ((SELECT id FROM projects WHERE title = 'say_data'), 'Data Engineering'),
  ((SELECT id FROM projects WHERE title = 'say_data'), 'Analytics');
