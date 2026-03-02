-- ═══════════════════════════════════════════════════════════════════════════════
-- migrations/0001_projects.sql — Dev page content
-- ───────────────────────────────────────────────────────────────────────────────
-- status:  'active' | 'building' | 'planning' | 'archived'
-- featured: shown prominently on home page stack section
-- published: controls visibility to visitors
-- sort_order: lower = first (manual ordering)
-- ═══════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'planning' CHECK (
    status IN ('active', 'building', 'planning', 'archived')
  ),
  repo_url TEXT,
  live_url TEXT,
  featured INTEGER NOT NULL DEFAULT 0,
  -- bool (0 / 1)
  published INTEGER NOT NULL DEFAULT 0,
  -- bool (0 / 1)
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
-- Tags are stored as plain text in a junction table.
-- Queried via GROUP_CONCAT so no extra round-trips per project.
CREATE TABLE IF NOT EXISTS project_tags (
  project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (project_id, tag)
);
-- Keep updated_at accurate without application-level code.
CREATE TRIGGER IF NOT EXISTS projects_updated_at
AFTER
UPDATE ON projects FOR EACH ROW BEGIN
UPDATE projects
SET updated_at = datetime('now')
WHERE id = OLD.id;
END;
