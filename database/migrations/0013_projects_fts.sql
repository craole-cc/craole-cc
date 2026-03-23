-- ═══════════════════════════════════════════════════════════════════════════════
-- FTS5 virtual table for full-text search on projects.
-- Mirrors title + description from the projects table.
-- Content table mode keeps data in sync without duplication.
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE VIRTUAL TABLE IF NOT EXISTS projects_fts USING fts5(
  title,
  description,
  content='projects',
  content_rowid='id'
);

-- Populate from existing published projects
INSERT INTO projects_fts (rowid, title, description)
SELECT id, title, description
FROM projects
WHERE published = 1;

-- Keep FTS in sync with projects via triggers
CREATE TRIGGER IF NOT EXISTS projects_fts_insert
AFTER INSERT ON projects
WHEN NEW.published = 1
BEGIN
  INSERT INTO projects_fts (rowid, title, description)
  VALUES (NEW.id, NEW.title, NEW.description);
END;

CREATE TRIGGER IF NOT EXISTS projects_fts_update
AFTER UPDATE ON projects
BEGIN
  DELETE FROM projects_fts WHERE rowid = OLD.id;
  INSERT INTO projects_fts (rowid, title, description)
  VALUES (NEW.id, NEW.title, NEW.description);
END;

CREATE TRIGGER IF NOT EXISTS projects_fts_delete
AFTER DELETE ON projects
BEGIN
  DELETE FROM projects_fts WHERE rowid = OLD.id;
END;
