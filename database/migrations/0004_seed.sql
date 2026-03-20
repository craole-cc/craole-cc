-- ═══════════════════════════════════════════════════════════════════════════════
-- migrations/0004_seed.sql — Initial content seeded from static data
-- ───────────────────────────────────────────────────────────────────────────────
-- Mirrors the hard-coded data in app/src/pages/home/projects.rs and
-- app/src/pages/home/experience.rs. Swap these for real content whenever ready.
-- ═══════════════════════════════════════════════════════════════════════════════
-- ── Projects ──────────────────────────────────────────────────────────────────
INSERT INTO projects (
    title,
    description,
    status,
    live_url,
    featured,
    published,
    sort_order
  )
VALUES (
    'Craole.CC — Portfolio',
    'Rust-powered portfolio site built with Leptos + Axum. Full-stack SSR with hydration, dynamic theming from photo hue extraction, and a custom icon registry.',
    'active',
    'https://craole.cc',
    1,
    1,
    10
  ),
  (
    'Data Pipeline System',
    'High-performance ingestion and transformation pipeline using Rust and Delta Lake. Designed for analytics workloads at scale.',
    'building',
    NULL,
    0,
    1,
    20
  ),
  (
    'Graph Analytics Platform',
    'Relationship-modelling platform built on Neo4j and Cypher. Surfaces patterns in business data that relational queries miss.',
    'building',
    NULL,
    0,
    1,
    30
  ),
  (
    'CLI Utilities Suite',
    'Cross-platform developer productivity tools and system utilities. Single static binaries targeting Windows, Linux, and macOS.',
    'planning',
    NULL,
    0,
    1,
    40
  );
-- Tags for the projects above (id order matches INSERT order above)
INSERT INTO project_tags (project_id, tag)
VALUES (1, 'Rust'),
  (1, 'Leptos'),
  (1, 'Axum'),
  (1, 'SQLite'),
  (2, 'Rust'),
  (2, 'Delta Lake'),
  (2, 'Apache Spark'),
  (3, 'Neo4j'),
  (3, 'Cypher'),
  (3, 'Rust'),
  (4, 'Rust'),
  (4, 'CLI'),
  (4, 'Cross-platform');
-- ── Posts ─────────────────────────────────────────────────────────────────────
INSERT INTO posts (
    title,
    slug,
    excerpt,
    kind,
    published,
    published_at
  )
VALUES (
    'Why I chose Leptos over Next.js',
    'why-leptos-over-nextjs',
    'Rust''s ownership model solves a class of UI bugs at compile time that JavaScript frameworks fight at runtime. Here''s how that played out in practice.',
    'blog',
    0,
    -- draft — publish when ready
    NULL
  ),
  (
    'Curriculum Vitae',
    'cv',
    'Software engineer and data specialist with roots in music production and Learning & Development.',
    'cv',
    1,
    datetime('now')
  );
-- ── Media ─────────────────────────────────────────────────────────────────────
-- Placeholder row — replace file_path with a real asset under public/media/
INSERT INTO media (
    title,
    caption,
    media_type,
    file_path,
    alt_text,
    published,
    sort_order
  )
VALUES (
    'Placeholder',
    'Replace with a real photo path under public/media/',
    'photo',
    'media/placeholder.jpg',
    'Placeholder image',
    0,
    -- unpublished until a real file exists
    10
  );
