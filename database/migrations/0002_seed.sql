-- ═══════════════════════════════════════════════════════════════════════════════
-- 0002_seed.sql — Initial content
-- ═══════════════════════════════════════════════════════════════════════════════
-- ── Projects ──────────────────────────────────────────────────────────────────
INSERT INTO
  projects (
    title,
    slug,
    description,
    status,
    repo_url,
    live_url,
    featured,
    published,
    sort_order
  )
VALUES
  (
    'craole.cc',
    'craolecс',
    'Full-stack portfolio and blog built entirely in Rust. Leptos handles reactive SSR with hydration, Axum serves the backend, SQLite stores content, and Tailwind + SCSS style every pixel. The site you''re looking at right now.',
    'active',
    'https://github.com/craole-cc/craole-cc',
    'https://craole.cc',
    1,
    1,
    10
  ),
  (
    'wallter',
    'wallter',
    'Dynamic desktop wallpaper manager that rotates backgrounds on a schedule. Cross-platform CLI built in Rust with a focus on clean argument parsing and OS-native integrations for Linux, macOS, and Windows.',
    'active',
    'https://github.com/craole-cc/wallter',
    NULL,
    1,
    1,
    20
  ),
  (
    'datum',
    'datum',
    'Unified data engineering lab for building pipelines in both Rust and Python. Explores Delta Lake for versioned storage, medallion architecture (bronze/silver/gold), and practical patterns for ingestion, transformation, and analytics.',
    'building',
    'https://github.com/craole-cc/datum',
    NULL,
    1,
    1,
    30
  ),
  (
    'ephelog',
    'ephelog',
    'Ephemeral logger that wraps the tracing ecosystem with sane defaults. Zero-config startup for CLI tools and servers — structured output, level filtering, and coloured terminal formatting out of the box.',
    'building',
    'https://github.com/craole-cc/ephelog',
    NULL,
    1,
    1,
    40
  ),
  (
    'dotDots',
    'dotdots',
    'Declarative system configuration for NixOS, GNOME, and development environments. Manages shell configs, editor setups, and reproducible toolchains across multiple machines — infrastructure-as-code for the desktop.',
    'active',
    'https://github.com/craole-cc/dotDots',
    NULL,
    0,
    1,
    50
  ),
  (
    'ccutils',
    'ccutils',
    'Collection of cross-platform shell utilities and developer productivity scripts. POSIX-compatible wrappers, environment helpers, and automation tools that smooth out day-to-day terminal workflows.',
    'active',
    'https://github.com/craole-cc/ccutils',
    NULL,
    0,
    1,
    60
  ),
  (
    'gitsy',
    'gitsy',
    'Git repository initialiser with opinionated defaults. Sets up commit conventions, branch structure, and remote configuration in a single command — removing friction from starting new projects.',
    'planning',
    'https://github.com/craole-cc/gitsy',
    NULL,
    0,
    1,
    70
  ),
  (
    'Advent of Code',
    'advent-of-code',
    'Solutions to Advent of Code challenges, implemented in Rust with a focus on idiomatic patterns, performance, and readable problem decomposition.',
    'active',
    'https://github.com/craole-cc/Advent-of-Code',
    NULL,
    0,
    1,
    80
  ),
  (
    'say_data',
    'say-data',
    'Exploration of data storytelling and analytics communication. Building tools and techniques for presenting data insights clearly — because the pipeline is only half the story.',
    'planning',
    'https://github.com/craole-cc/say_data',
    NULL,
    0,
    1,
    90
  );

INSERT INTO
  project_tags (project_id, tag)
VALUES
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'Leptos'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'Axum'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'SQLite'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'Tailwind'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'craolecс'
    ),
    'Full-Stack'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'wallter'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'wallter'
    ),
    'CLI'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'wallter'
    ),
    'Cross-Platform'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'wallter'
    ),
    'Desktop'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'datum'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'datum'
    ),
    'Python'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'datum'
    ),
    'Delta Lake'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'datum'
    ),
    'Data Engineering'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ephelog'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ephelog'
    ),
    'Tracing'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ephelog'
    ),
    'CLI'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ephelog'
    ),
    'Open Source'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'dotdots'
    ),
    'Nix'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'dotdots'
    ),
    'NixOS'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'dotdots'
    ),
    'Shell'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'dotdots'
    ),
    'Infrastructure'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ccutils'
    ),
    'Shell'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ccutils'
    ),
    'POSIX'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ccutils'
    ),
    'CLI'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'ccutils'
    ),
    'Cross-Platform'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'gitsy'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'gitsy'
    ),
    'Git'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'gitsy'
    ),
    'CLI'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'advent-of-code'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'advent-of-code'
    ),
    'Algorithms'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'say-data'
    ),
    'Rust'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'say-data'
    ),
    'Data Engineering'
  ),
  (
    (
      SELECT
        id
      FROM
        projects
      WHERE
        slug = 'say-data'
    ),
    'Analytics'
  );

INSERT INTO
  projects_fts (rowid, title, description)
SELECT
  id,
  title,
  description
FROM
  projects
WHERE
  published = 1;

-- ── Posts ─────────────────────────────────────────────────────────────────────
INSERT INTO
  posts (
    title,
    slug,
    excerpt,
    kind,
    featured,
    cover_url,
    published,
    published_at,
    body
  )
VALUES
  (
    'Curriculum Vitae',
    'cv',
    'Software engineer and data specialist with roots in music production and Learning & Development.',
    'cv',
    0,
    'https://images.unsplash.com/photo-1511379938547-c1f69419868d?auto=format&fit=crop&w=1200&q=80',
    1,
    datetime('now'),
    '## Craig "Craole" Cole

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

Rust · Leptos · Axum · SQLite · Neo4j · Delta Lake · NixOS · Linux'
  ),
  (
    'Why I chose Leptos over Next.js',
    'why-leptos-over-nextjs',
    'Rust''s ownership model solves a class of UI bugs at compile time that JavaScript frameworks fight at runtime. Here''s how that played out in practice.',
    'blog',
    0,
    'https://images.unsplash.com/photo-1593720213428-28a5b9e94613?auto=format&fit=crop&w=1200&q=80',
    1,
    datetime('now', '-2 days'),
    '## Why Rust for the Web

The question I get most often is: why not just use Next.js?...'
  ),
  (
    'Raised on Rhythm: How Music Shaped My Approach to Code',
    'music-and-code',
    'There is a reason jazz musicians make great engineers. Both disciplines demand improvisation within structure, creativity within constraint.',
    'blog',
    0,
    'https://images.unsplash.com/photo-1510915361894-db8b60106cb1?auto=format&fit=crop&w=1200&q=80',
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
    0,
    'https://images.unsplash.com/photo-1547036967-23d11aacaee0?auto=format&fit=crop&w=1200&q=80',
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
    0,
    'https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&w=1200&q=80',
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
    0,
    'https://images.unsplash.com/photo-1507838153414-b4b713384a76?auto=format&fit=crop&w=1200&q=80',
    1,
    datetime('now', '-35 days'),
    '## The Discovery

It started as an accident...'
  );

INSERT INTO
  posts_fts (rowid, title, excerpt, body)
SELECT
  id,
  title,
  coalesce(excerpt, ''),
  coalesce(body, '')
FROM
  posts
WHERE
  published = 1;

-- ── Media ─────────────────────────────────────────────────────────────────────
INSERT INTO
  media (
    title,
    slug,
    caption,
    media_type,
    file_path,
    alt_text,
    width,
    height,
    published,
    sort_order,
    taken_at
  )
VALUES
  (
    'Blue Mountains Morning',
    'blue-mountains-morning',
    'Mist rolling through the Blue Mountains at first light. The valley holds its breath before the sun arrives.',
    'photo',
    'https://images.unsplash.com/photo-1501854140801-50d01698950b?auto=format&fit=crop&w=1920&q=80',
    'Misty mountain valley at dawn with layers of blue ridges',
    1920,
    1280,
    1,
    10,
    '2024-03-15'
  ),
  (
    'Harbour at Dusk',
    'harbour-at-dusk',
    'The moment the light dies and the city wakes. Kingston harbour holds both worlds at once.',
    'photo',
    'https://images.unsplash.com/photo-1507525428034-b723cf961d3e?auto=format&fit=crop&w=1920&q=80',
    'Tropical beach at golden hour with calm water',
    1920,
    1280,
    1,
    20,
    '2024-02-20'
  ),
  (
    'Cascade',
    'cascade',
    'Dunns River. Every tourist has a photo of this place. Somehow it still earns one.',
    'photo',
    'https://images.unsplash.com/photo-1546587348-d12660c30c50?auto=format&fit=crop&w=1920&q=80',
    'Waterfall cascading over moss-covered rocks',
    1280,
    1920,
    1,
    30,
    '2024-01-10'
  ),
  (
    'The Quiet Road',
    'the-quiet-road',
    'A parish road at 5am. Jamaica before it wakes up is a different country entirely.',
    'photo',
    'https://images.unsplash.com/photo-1469474968028-56623f02e42e?auto=format&fit=crop&w=1920&q=80',
    'Empty road through mountain landscape at dawn',
    1920,
    1280,
    1,
    40,
    '2024-01-08'
  ),
  (
    'Reef Study',
    'reef-study',
    'Beneath the surface everything slows down. Colour, shape, time.',
    'photo',
    'https://images.unsplash.com/photo-1559128010-7c1ad6e1b6a5?auto=format&fit=crop&w=1920&q=80',
    'Underwater coral reef with tropical fish',
    1920,
    1280,
    1,
    50,
    '2023-12-05'
  ),
  (
    'Night Code',
    'night-code',
    'The terminal at 2am. Most of the good ideas happen here.',
    'photo',
    'https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&w=1920&q=80',
    'Code editor with syntax highlighting on dark background',
    1920,
    1080,
    1,
    60,
    '2024-03-01'
  ),
  (
    'Studio Session',
    'studio-session',
    'Before the track had a name. The room when it is still deciding what it wants to be.',
    'photo',
    'https://images.unsplash.com/photo-1510915361894-db8b60106cb1?auto=format&fit=crop&w=1920&q=80',
    'Music studio with guitar and recording equipment',
    1920,
    1280,
    1,
    70,
    '2023-11-20'
  ),
  (
    'Concrete and Sky',
    'concrete-and-sky',
    'New Kingston. Glass and steel learning to live alongside everything else.',
    'photo',
    'https://images.unsplash.com/photo-1486325212027-8081e485255e?auto=format&fit=crop&w=1920&q=80',
    'Modern city buildings against blue sky',
    1920,
    1280,
    1,
    80,
    '2024-02-14'
  ),
  (
    'Salt Pond',
    'salt-pond',
    'The flamingos were unimpressed by the camera. As they should be.',
    'photo',
    'https://images.unsplash.com/photo-1559825481-12a05cc00344?auto=format&fit=crop&w=1920&q=80',
    'Flamingos standing in a shallow salt lake',
    1920,
    1280,
    1,
    90,
    '2023-10-30'
  ),
  (
    'Root System',
    'root-system',
    'A silk cotton tree in St. Thomas. Some things take three hundred years to become themselves.',
    'photo',
    'https://images.unsplash.com/photo-1448375240586-882707db888b?auto=format&fit=crop&w=1920&q=80',
    'Massive tree roots spreading across forest floor',
    1280,
    1920,
    1,
    100,
    '2023-09-15'
  ),
  (
    'Typewriter',
    'typewriter',
    'My grandfather''s. Still works. The keys require commitment.',
    'photo',
    'https://images.unsplash.com/photo-1471107340929-a87cd0f5b5f3?auto=format&fit=crop&w=1920&q=80',
    'Vintage typewriter keys close up',
    1920,
    1280,
    1,
    110,
    '2023-08-22'
  ),
  (
    'Long Exposure, Kingston',
    'long-exposure-kingston',
    'Eight seconds. The cars became light. The city became abstraction.',
    'photo',
    'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?auto=format&fit=crop&w=1920&q=80',
    'City lights streaking in long exposure night photography',
    1920,
    1280,
    1,
    120,
    '2024-03-10'
  );

INSERT INTO
  media_tags (media_id, tag)
VALUES
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'blue-mountains-morning'
    ),
    'landscape'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'blue-mountains-morning'
    ),
    'mountains'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'blue-mountains-morning'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'blue-mountains-morning'
    ),
    'dawn'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'harbour-at-dusk'
    ),
    'seascape'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'harbour-at-dusk'
    ),
    'golden-hour'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'harbour-at-dusk'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'harbour-at-dusk'
    ),
    'water'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'cascade'
    ),
    'waterfall'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'cascade'
    ),
    'nature'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'cascade'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'cascade'
    ),
    'water'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'the-quiet-road'
    ),
    'landscape'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'the-quiet-road'
    ),
    'dawn'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'the-quiet-road'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'the-quiet-road'
    ),
    'roads'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'reef-study'
    ),
    'underwater'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'reef-study'
    ),
    'nature'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'reef-study'
    ),
    'water'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'reef-study'
    ),
    'colour'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'night-code'
    ),
    'technology'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'night-code'
    ),
    'code'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'night-code'
    ),
    'night'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'studio-session'
    ),
    'music'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'studio-session'
    ),
    'studio'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'studio-session'
    ),
    'night'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'concrete-and-sky'
    ),
    'urban'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'concrete-and-sky'
    ),
    'architecture'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'concrete-and-sky'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'salt-pond'
    ),
    'wildlife'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'salt-pond'
    ),
    'nature'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'salt-pond'
    ),
    'colour'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'root-system'
    ),
    'nature'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'root-system'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'root-system'
    ),
    'trees'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'typewriter'
    ),
    'still-life'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'typewriter'
    ),
    'vintage'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'typewriter'
    ),
    'writing'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'long-exposure-kingston'
    ),
    'urban'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'long-exposure-kingston'
    ),
    'night'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'long-exposure-kingston'
    ),
    'jamaica'
  ),
  (
    (
      SELECT
        id
      FROM
        media
      WHERE
        slug = 'long-exposure-kingston'
    ),
    'light'
  );

INSERT INTO
  media_fts (rowid, title, caption, alt_text)
SELECT
  id,
  title,
  coalesce(caption, ''),
  alt_text
FROM
  media
WHERE
  published = 1;
