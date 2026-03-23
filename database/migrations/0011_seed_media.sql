-- Remove placeholder
DELETE FROM media WHERE file_path = 'media/placeholder.jpg';

-- ── Photos ────────────────────────────────────────────────────────────────────

INSERT INTO media (title, slug, caption, media_type, file_path, alt_text, width, height, published, sort_order, taken_at) VALUES
(
  'Blue Mountains Morning',
  'blue-mountains-morning',
  'Mist rolling through the Blue Mountains at first light. The valley holds its breath before the sun arrives.',
  'photo',
  'https://images.unsplash.com/photo-1501854140801-50d01698950b?auto=format&fit=crop&w=1920&q=80',
  'Misty mountain valley at dawn with layers of blue ridges',
  1920, 1280, 1, 10, '2024-03-15'
),
(
  'Harbour at Dusk',
  'harbour-at-dusk',
  'The moment the light dies and the city wakes. Kingston harbour holds both worlds at once.',
  'photo',
  'https://images.unsplash.com/photo-1507525428034-b723cf961d3e?auto=format&fit=crop&w=1920&q=80',
  'Tropical beach at golden hour with calm water',
  1920, 1280, 1, 20, '2024-02-20'
),
(
  'Cascade',
  'cascade',
  'Dunns River. Every tourist has a photo of this place. Somehow it still earns one.',
  'photo',
  'https://images.unsplash.com/photo-1546587348-d12660c30c50?auto=format&fit=crop&w=1920&q=80',
  'Waterfall cascading over moss-covered rocks',
  1280, 1920, 1, 30, '2024-01-10'
),
(
  'The Quiet Road',
  'the-quiet-road',
  'A parish road at 5am. Jamaica before it wakes up is a different country entirely.',
  'photo',
  'https://images.unsplash.com/photo-1469474968028-56623f02e42e?auto=format&fit=crop&w=1920&q=80',
  'Empty road through mountain landscape at dawn',
  1920, 1280, 1, 40, '2024-01-08'
),
(
  'Reef Study',
  'reef-study',
  'Beneath the surface everything slows down. Colour, shape, time.',
  'photo',
  'https://images.unsplash.com/photo-1559128010-7c1ad6e1b6a5?auto=format&fit=crop&w=1920&q=80',
  'Underwater coral reef with tropical fish',
  1920, 1280, 1, 50, '2023-12-05'
),
(
  'Night Code',
  'night-code',
  'The terminal at 2am. Most of the good ideas happen here.',
  'photo',
  'https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&w=1920&q=80',
  'Code editor with syntax highlighting on dark background',
  1920, 1080, 1, 60, '2024-03-01'
),
(
  'Studio Session',
  'studio-session',
  'Before the track had a name. The room when it is still deciding what it wants to be.',
  'photo',
  'https://images.unsplash.com/photo-1510915361894-db8b60106cb1?auto=format&fit=crop&w=1920&q=80',
  'Music studio with guitar and recording equipment',
  1920, 1280, 1, 70, '2023-11-20'
),
(
  'Concrete and Sky',
  'concrete-and-sky',
  'New Kingston. Glass and steel learning to live alongside everything else.',
  'photo',
  'https://images.unsplash.com/photo-1486325212027-8081e485255e?auto=format&fit=crop&w=1920&q=80',
  'Modern city buildings against blue sky',
  1920, 1280, 1, 80, '2024-02-14'
),
(
  'Salt Pond',
  'salt-pond',
  'The flamingos were unimpressed by the camera. As they should be.',
  'photo',
  'https://images.unsplash.com/photo-1559825481-12a05cc00344?auto=format&fit=crop&w=1920&q=80',
  'Flamingos standing in a shallow salt lake',
  1920, 1280, 1, 90, '2023-10-30'
),
(
  'Root System',
  'root-system',
  'A silk cotton tree in St. Thomas. Some things take three hundred years to become themselves.',
  'photo',
  'https://images.unsplash.com/photo-1448375240586-882707db888b?auto=format&fit=crop&w=1920&q=80',
  'Massive tree roots spreading across forest floor',
  1280, 1920, 1, 100, '2023-09-15'
),
(
  'Typewriter',
  'typewriter',
  'My grandfather''s. Still works. The keys require commitment.',
  'photo',
  'https://images.unsplash.com/photo-1471107340929-a87cd0f5b5f3?auto=format&fit=crop&w=1920&q=80',
  'Vintage typewriter keys close up',
  1920, 1280, 1, 110, '2023-08-22'
),
(
  'Long Exposure, Kingston',
  'long-exposure-kingston',
  'Eight seconds. The cars became light. The city became abstraction.',
  'photo',
  'https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?auto=format&fit=crop&w=1920&q=80',
  'City lights streaking in long exposure night photography',
  1920, 1280, 1, 120, '2024-03-10'
);

-- ── Tags ──────────────────────────────────────────────────────────────────────

-- Blue Mountains Morning
INSERT INTO media_tags (media_id, tag) SELECT id, 'landscape' FROM media WHERE title = 'Blue Mountains Morning';
INSERT INTO media_tags (media_id, tag) SELECT id, 'mountains' FROM media WHERE title = 'Blue Mountains Morning';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'Blue Mountains Morning';
INSERT INTO media_tags (media_id, tag) SELECT id, 'dawn' FROM media WHERE title = 'Blue Mountains Morning';

-- Harbour at Dusk
INSERT INTO media_tags (media_id, tag) SELECT id, 'seascape' FROM media WHERE title = 'Harbour at Dusk';
INSERT INTO media_tags (media_id, tag) SELECT id, 'golden-hour' FROM media WHERE title = 'Harbour at Dusk';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica'  FROM media WHERE title = 'Harbour at Dusk';
INSERT INTO media_tags (media_id, tag) SELECT id, 'water'  FROM media WHERE title = 'Harbour at Dusk';

-- Cascade
INSERT INTO media_tags (media_id, tag) SELECT id, 'waterfall' FROM media WHERE title = 'Cascade';
INSERT INTO media_tags (media_id, tag) SELECT id, 'nature' FROM media WHERE title = 'Cascade';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'Cascade';
INSERT INTO media_tags (media_id, tag) SELECT id, 'water'  FROM media WHERE title = 'Cascade';

-- The Quiet Road
INSERT INTO media_tags (media_id, tag) SELECT id, 'landscape' FROM media WHERE title = 'The Quiet Road';
INSERT INTO media_tags (media_id, tag) SELECT id, 'dawn' FROM media WHERE title = 'The Quiet Road';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'The Quiet Road';
INSERT INTO media_tags (media_id, tag) SELECT id, 'roads'  FROM media WHERE title = 'The Quiet Road';

-- Reef Study
INSERT INTO media_tags (media_id, tag) SELECT id, 'underwater' FROM media WHERE title = 'Reef Study';
INSERT INTO media_tags (media_id, tag) SELECT id, 'nature'  FROM media WHERE title = 'Reef Study';
INSERT INTO media_tags (media_id, tag) SELECT id, 'water' FROM media WHERE title = 'Reef Study';
INSERT INTO media_tags (media_id, tag) SELECT id, 'colour'  FROM media WHERE title = 'Reef Study';

-- Night Code
INSERT INTO media_tags (media_id, tag) SELECT id, 'technology' FROM media WHERE title = 'Night Code';
INSERT INTO media_tags (media_id, tag) SELECT id, 'code'  FROM media WHERE title = 'Night Code';
INSERT INTO media_tags (media_id, tag) SELECT id, 'night' FROM media WHERE title = 'Night Code';

-- Studio Session
INSERT INTO media_tags (media_id, tag) SELECT id, 'music' FROM media WHERE title = 'Studio Session';
INSERT INTO media_tags (media_id, tag) SELECT id, 'studio' FROM media WHERE title = 'Studio Session';
INSERT INTO media_tags (media_id, tag) SELECT id, 'night' FROM media WHERE title = 'Studio Session';

-- Concrete and Sky
INSERT INTO media_tags (media_id, tag) SELECT id, 'urban'  FROM media WHERE title = 'Concrete and Sky';
INSERT INTO media_tags (media_id, tag) SELECT id, 'architecture' FROM media WHERE title = 'Concrete and Sky';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'Concrete and Sky';

-- Salt Pond
INSERT INTO media_tags (media_id, tag) SELECT id, 'wildlife' FROM media WHERE title = 'Salt Pond';
INSERT INTO media_tags (media_id, tag) SELECT id, 'nature' FROM media WHERE title = 'Salt Pond';
INSERT INTO media_tags (media_id, tag) SELECT id, 'colour' FROM media WHERE title = 'Salt Pond';

-- Root System
INSERT INTO media_tags (media_id, tag) SELECT id, 'nature' FROM media WHERE title = 'Root System';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'Root System';
INSERT INTO media_tags (media_id, tag) SELECT id, 'trees' FROM media WHERE title = 'Root System';

-- Typewriter
INSERT INTO media_tags (media_id, tag) SELECT id, 'still-life' FROM media WHERE title = 'Typewriter';
INSERT INTO media_tags (media_id, tag) SELECT id, 'vintage' FROM media WHERE title = 'Typewriter';
INSERT INTO media_tags (media_id, tag) SELECT id, 'writing' FROM media WHERE title = 'Typewriter';

-- Long Exposure, Kingston
INSERT INTO media_tags (media_id, tag) SELECT id, 'urban' FROM media WHERE title = 'Long Exposure, Kingston';
INSERT INTO media_tags (media_id, tag) SELECT id, 'night' FROM media WHERE title = 'Long Exposure, Kingston';
INSERT INTO media_tags (media_id, tag) SELECT id, 'jamaica' FROM media WHERE title = 'Long Exposure, Kingston';
INSERT INTO media_tags (media_id, tag) SELECT id, 'light' FROM media WHERE title = 'Long Exposure, Kingston';
