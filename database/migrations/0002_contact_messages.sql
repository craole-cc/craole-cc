-- Contact form submissions are private operational data.
-- Keep this table out of public content/search indexes.
CREATE TABLE IF NOT EXISTS contact_messages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  subject TEXT NOT NULL,
  message TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  read_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_contact_messages_created_at
  ON contact_messages (created_at DESC);
