-- Additional dummy posts for UI development
UPDATE posts SET published = 1 WHERE slug = 'why-leptos-over-nextjs';

INSERT INTO posts (title, slug, excerpt, kind, published, published_at, body) VALUES
(
  'Raised on Rhythm: How Music Shaped My Approach to Code',
  'music-and-code',
  'There is a reason jazz musicians make great engineers. Both disciplines demand improvisation within structure, creativity within constraint.',
  'blog',
  1,
  datetime('now', '-5 days'),
  '## The Connection

When I picked up the bass at fourteen, I had no idea I was learning to think like a programmer...'
),
(
  'NixOS: Declarative Infrastructure as a Way of Life',
  'nixos-declarative-infrastructure',
  'What happens when you treat your entire operating system like a version-controlled configuration file? Reproducibility, sanity, and the occasional existential crisis.',
  'blog',
  1,
  datetime('now', '-12 days'),
  '## Why Nix

The promise is simple: describe your system, and it will be exactly that system...'
),
(
  'Building a Portfolio in Rust: Lessons from the Trenches',
  'portfolio-in-rust',
  'Nobody said it would be easy. Leptos, Axum, SQLx, WASM — a full-stack Rust portfolio is either madness or the future. Possibly both.',
  'blog',
  1,
  datetime('now', '-20 days'),
  '## Why Rust for the Web

The question I get most often is: why not just use Next.js?...'
),
(
  'Teaching English Through Rhythm and Melody',
  'teaching-english-rhythm',
  'After three years teaching TEFL, the single most effective technique I found had nothing to do with grammar. It had everything to do with music.',
  'note',
  1,
  datetime('now', '-35 days'),
  '## The Discovery

It started as an accident...'
);

-- Update CV body
UPDATE posts SET body = '## Craig "Craole" Cole

Software engineer and data specialist with roots in music production and Learning & Development.

## Experience

### Data/Backend Developer
*2024 – Present*

Building modern data pipelines with Rust and Delta Lake. Exploring graph databases and advanced SQL patterns.

### Learning & Development Specialist
*8+ years*

Built training programs and analytics dashboards. Specialized in Power BI and Tableau for business intelligence.

### TEFL Tutor
*3 years*

Teaching professionals and students English communication. Developing custom learning materials.

## Skills

Rust · Leptos · Axum · SQLite · Neo4j · Delta Lake · NixOS · Linux' WHERE slug = 'cv';
