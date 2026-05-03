-- ═══════════════════════════════════════════════════════════════════════════════
-- 0001_schema.sql — Full database schema
-- ═══════════════════════════════════════════════════════════════════════════════
-- ── Projects ──────────────────────────────────────────────────────────────────
-- status:      'active' | 'building' | 'planning' | 'archived'
-- featured:    shown on home page stack section
-- published:   controls visitor visibility
-- sort_order:  lower = first (manual ordering)
-- slug:        clean URL identifier — /dev/{slug}
-- readme_html: cached GitHub README, rendered HTML
-- screenshots: comma-separated URLs, fallback to README images
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT UNIQUE,
  description TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'planning' CHECK (
    status IN ('active', 'building', 'planning', 'archived')
  ),
  repo_url TEXT,
  live_url TEXT,
  readme_html TEXT,
  screenshots TEXT,
  featured INTEGER NOT NULL DEFAULT 0,
  published INTEGER NOT NULL DEFAULT 0,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS project_tags (
  project_id INTEGER NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (project_id, tag)
);

CREATE TRIGGER IF NOT EXISTS projects_updated_at AFTER
UPDATE ON projects FOR EACH ROW BEGIN
UPDATE projects
SET
  updated_at = datetime('now')
WHERE
  id = OLD.id;

END;

CREATE VIRTUAL TABLE IF NOT EXISTS projects_fts USING fts5 (
  title,
  description,
  content = 'projects',
  content_rowid = 'id'
);

CREATE TRIGGER IF NOT EXISTS projects_fts_insert AFTER INSERT ON projects WHEN NEW.published = 1 BEGIN
INSERT INTO
  projects_fts (rowid, title, description)
VALUES
  (NEW.id, NEW.title, NEW.description);

END;

CREATE TRIGGER IF NOT EXISTS projects_fts_update AFTER
UPDATE ON projects BEGIN
DELETE FROM projects_fts
WHERE
  rowid = OLD.id;

INSERT INTO
  projects_fts (rowid, title, description)
VALUES
  (NEW.id, NEW.title, NEW.description);

END;

CREATE TRIGGER IF NOT EXISTS projects_fts_delete AFTER DELETE ON projects BEGIN
DELETE FROM projects_fts
WHERE
  rowid = OLD.id;

END;

-- ── Media ─────────────────────────────────────────────────────────────────────
-- media_type: 'photo' | 'video'
-- file_path:  relative to public/ — e.g. "media/photos/2024/blue-mountains.jpg"
-- taken_at:   ISO-8601, nullable (scanned/undated material)
-- width/height: pixel dimensions for aspect-ratio layout hints
CREATE TABLE IF NOT EXISTS media (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE DEFAULT '',
  caption TEXT,
  media_type TEXT NOT NULL DEFAULT 'photo' CHECK (media_type IN ('photo', 'video')),
  file_path TEXT NOT NULL UNIQUE,
  alt_text TEXT NOT NULL DEFAULT '',
  width INTEGER,
  height INTEGER,
  published INTEGER NOT NULL DEFAULT 0,
  sort_order INTEGER NOT NULL DEFAULT 0,
  taken_at TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS media_tags (
  media_id INTEGER NOT NULL REFERENCES media (id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (media_id, tag)
);

CREATE VIRTUAL TABLE IF NOT EXISTS media_fts USING fts5 (
  title,
  caption,
  alt_text,
  content = 'media',
  content_rowid = 'id'
);

CREATE TRIGGER IF NOT EXISTS media_fts_insert AFTER INSERT ON media WHEN NEW.published = 1 BEGIN
INSERT INTO
  media_fts (rowid, title, caption, alt_text)
VALUES
  (
    NEW.id,
    NEW.title,
    COALESCE(NEW.caption, ''),
    NEW.alt_text
  );

END;

CREATE TRIGGER IF NOT EXISTS media_fts_update AFTER
UPDATE ON media BEGIN
DELETE FROM media_fts
WHERE
  rowid = OLD.id;

INSERT INTO
  media_fts (rowid, title, caption, alt_text)
VALUES
  (
    NEW.id,
    NEW.title,
    COALESCE(NEW.caption, ''),
    NEW.alt_text
  );

END;

CREATE TRIGGER IF NOT EXISTS media_fts_delete AFTER DELETE ON media BEGIN
DELETE FROM media_fts
WHERE
  rowid = OLD.id;

END;

-- ── Posts ─────────────────────────────────────────────────────────────────────
-- slug:         URL-safe identifier — /log/{slug}
-- body:         raw Markdown
-- kind:         'blog' | 'cv' | 'note'
-- excerpt:      short preview (auto-derived if NULL)
-- featured:     pinned/highlighted in list views
-- cover_url:    hero image URL
-- published_at: public visibility date — may differ from created_at
CREATE TABLE IF NOT EXISTS posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  body TEXT NOT NULL DEFAULT '',
  excerpt TEXT,
  kind TEXT NOT NULL DEFAULT 'blog' CHECK (kind IN ('blog', 'cv', 'note')),
  featured INTEGER NOT NULL DEFAULT 0,
  cover_url TEXT,
  published INTEGER NOT NULL DEFAULT 0,
  published_at TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS post_tags (
  post_id INTEGER NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (post_id, tag)
);

CREATE TRIGGER IF NOT EXISTS posts_updated_at AFTER
UPDATE ON posts FOR EACH ROW BEGIN
UPDATE posts
SET
  updated_at = datetime('now')
WHERE
  id = OLD.id;

END;

CREATE VIRTUAL TABLE IF NOT EXISTS posts_fts USING fts5 (
  title,
  excerpt,
  body,
  content = 'posts',
  content_rowid = 'id'
);

CREATE TRIGGER IF NOT EXISTS posts_fts_insert AFTER INSERT ON posts WHEN NEW.published = 1 BEGIN
INSERT INTO
  posts_fts (rowid, title, excerpt, body)
VALUES
  (
    NEW.id,
    NEW.title,
    COALESCE(NEW.excerpt, ''),
    COALESCE(NEW.body, '')
  );

END;

CREATE TRIGGER IF NOT EXISTS posts_fts_update AFTER
UPDATE ON posts BEGIN
DELETE FROM posts_fts
WHERE
  rowid = OLD.id;

INSERT INTO
  posts_fts (rowid, title, excerpt, body)
VALUES
  (
    NEW.id,
    NEW.title,
    COALESCE(NEW.excerpt, ''),
    COALESCE(NEW.body, '')
  );

END;

CREATE TRIGGER IF NOT EXISTS posts_fts_delete AFTER DELETE ON posts BEGIN
DELETE FROM posts_fts
WHERE
  rowid = OLD.id;

END;
