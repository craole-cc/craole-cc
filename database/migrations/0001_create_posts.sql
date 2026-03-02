CREATE TABLE IF NOT EXISTS posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  body TEXT NOT NULL DEFAULT '',
  excerpt TEXT,
  kind TEXT NOT NULL DEFAULT 'Blog',
  published_at TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  tags TEXT -- ← add this
);
