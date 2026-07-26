-- 0006_media_audio.sql — allow audio records in the media catalog.
-- SQLite cannot alter a CHECK constraint in place, so rebuild the two media tables
-- while preserving IDs, tags, and the FTS index.
DROP TRIGGER IF EXISTS media_fts_insert;
DROP TRIGGER IF EXISTS media_fts_update;
DROP TRIGGER IF EXISTS media_fts_delete;
DROP TABLE IF EXISTS media_fts;

CREATE TABLE media_new (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE DEFAULT '',
  caption TEXT,
  media_type TEXT NOT NULL DEFAULT 'photo' CHECK (media_type IN ('photo', 'video', 'audio')),
  file_path TEXT NOT NULL UNIQUE,
  alt_text TEXT NOT NULL DEFAULT '',
  width INTEGER,
  height INTEGER,
  published INTEGER NOT NULL DEFAULT 0,
  sort_order INTEGER NOT NULL DEFAULT 0,
  taken_at TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO media_new (id, title, slug, caption, media_type, file_path, alt_text, width, height, published, sort_order, taken_at, created_at)
SELECT id, title, slug, caption, media_type, file_path, alt_text, width, height, published, sort_order, taken_at, created_at
FROM media;

CREATE TABLE media_tags_new (
  media_id INTEGER NOT NULL REFERENCES media_new (id) ON DELETE CASCADE,
  tag TEXT NOT NULL,
  PRIMARY KEY (media_id, tag)
);
INSERT INTO media_tags_new (media_id, tag) SELECT media_id, tag FROM media_tags;

DROP TABLE media_tags;
DROP TABLE media;
ALTER TABLE media_new RENAME TO media;
ALTER TABLE media_tags_new RENAME TO media_tags;

CREATE VIRTUAL TABLE media_fts USING fts5 (
  title,
  caption,
  alt_text,
  content = 'media',
  content_rowid = 'id'
);

CREATE TRIGGER media_fts_insert AFTER INSERT ON media WHEN NEW.published = 1 BEGIN
  INSERT INTO media_fts (rowid, title, caption, alt_text)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.caption, ''), NEW.alt_text);
END;

CREATE TRIGGER media_fts_update AFTER UPDATE ON media BEGIN
  DELETE FROM media_fts WHERE rowid = OLD.id;
  INSERT INTO media_fts (rowid, title, caption, alt_text)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.caption, ''), NEW.alt_text);
END;

CREATE TRIGGER media_fts_delete AFTER DELETE ON media BEGIN
  DELETE FROM media_fts WHERE rowid = OLD.id;
END;

INSERT INTO media_fts (rowid, title, caption, alt_text)
SELECT id, title, COALESCE(caption, ''), alt_text FROM media WHERE published = 1;
