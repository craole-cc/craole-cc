-- Add featured flag to posts.
-- Default 0 — the view layer falls back to most recent when none are flagged.
ALTER TABLE posts ADD COLUMN featured INTEGER NOT NULL DEFAULT 0;
