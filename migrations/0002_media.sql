-- ═══════════════════════════════════════════════════════════════════════════════
-- migrations/0002_media.sql — Art page content
-- ───────────────────────────────────────────────────────────────────────────────
-- media_type: 'photo' | 'video'
-- file_path:  relative to public/ — e.g. "media/photos/2024/jamaica-beach.jpg"
-- taken_at:   ISO-8601 string, nullable (scanned/undated material)
-- width/height: pixel dimensions, used for aspect-ratio layout hints
-- ═══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS media (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  caption TEXT,
  media_type TEXT NOT NULL DEFAULT 'photo' CHECK (media_type IN ('photo', 'video')),
  file_path TEXT NOT NULL UNIQUE,
  -- relative to public/
  alt_text TEXT NOT NULL DEFAULT '',
  width INTEGER,
  height INTEGER,
  published INTEGER NOT NULL DEFAULT 0,
  -- bool (0 / 1)
  sort_order INTEGER NOT NULL DEFAULT 0,
  taken_at TEXT,
  -- ISO-8601 or NULL
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
