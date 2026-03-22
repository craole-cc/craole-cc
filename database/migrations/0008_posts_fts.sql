-- ═══════════════════════════════════════════════════════════════════════════════
-- FTS5 virtual table for full-text search on posts.
-- Mirrors title + excerpt + body from the posts table.
-- Content table mode keeps data in sync without duplication.
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE VIRTUAL TABLE IF NOT EXISTS posts_fts USING fts5(
  title,
  excerpt,
  body,
  content='posts',
  content_rowid='id'
);

-- Populate from existing published posts
INSERT INTO posts_fts (rowid, title, excerpt, body)
SELECT id, title, COALESCE(excerpt, ''), COALESCE(body, '')
FROM posts
WHERE published = 1;

-- Keep FTS in sync with posts via triggers
CREATE TRIGGER IF NOT EXISTS posts_fts_insert
AFTER INSERT ON posts
WHEN NEW.published = 1
BEGIN
  INSERT INTO posts_fts (rowid, title, excerpt, body)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.excerpt, ''), COALESCE(NEW.body, ''));
END;

CREATE TRIGGER IF NOT EXISTS posts_fts_update
AFTER UPDATE ON posts
BEGIN
  DELETE FROM posts_fts WHERE rowid = OLD.id;
  INSERT INTO posts_fts (rowid, title, excerpt, body)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.excerpt, ''), COALESCE(NEW.body, ''));
END;

CREATE TRIGGER IF NOT EXISTS posts_fts_delete
AFTER DELETE ON posts
BEGIN
  DELETE FROM posts_fts WHERE rowid = OLD.id;
END;
