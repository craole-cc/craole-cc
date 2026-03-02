-- ═══════════════════════════════════════════════════════════════════════════════
-- migrations/0003_posts.sql — Log page content (blog, cv entries, notes)
-- ───────────────────────────────────────────────────────────────────────────────
-- slug:         URL-safe identifier — must be unique
-- body:         raw Markdown — rendered client-side or at serve time
-- kind:         'blog' | 'cv' | 'note'
-- excerpt:      short preview shown in list views (auto-derived if NULL)
-- published_at: when the post went live — may differ from created_at
-- ═══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  body TEXT NOT NULL DEFAULT '',
  -- Markdown
  excerpt TEXT,
  kind TEXT NOT NULL DEFAULT 'blog' CHECK (kind IN ('blog', 'cv', 'note')),
  published INTEGER NOT NULL DEFAULT 0,
  -- bool (0 / 1)
  published_at TEXT,
  -- ISO-8601 or NULL
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
-- Tags shared with projects use the same flat-text pattern.
CREATE TABLE IF NOT EXISTS post_tags (
  post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (post_id, tag)
);
CREATE TRIGGER IF NOT EXISTS posts_updated_at
AFTER
UPDATE ON posts FOR EACH ROW BEGIN
UPDATE posts
SET updated_at = datetime('now')
WHERE id = OLD.id;
END;
