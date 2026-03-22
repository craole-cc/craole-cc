CREATE TABLE IF NOT EXISTS media_tags (
  media_id INTEGER NOT NULL REFERENCES media(id) ON DELETE CASCADE,
  tag      TEXT    NOT NULL,
  PRIMARY KEY (media_id, tag)
);

CREATE VIRTUAL TABLE IF NOT EXISTS media_fts USING fts5(
  title,
  caption,
  alt_text,
  content='media',
  content_rowid='id'
);

INSERT INTO media_fts (rowid, title, caption, alt_text)
SELECT id, title, COALESCE(caption, ''), alt_text
FROM media
WHERE published = 1;

CREATE TRIGGER IF NOT EXISTS media_fts_insert
AFTER INSERT ON media
WHEN NEW.published = 1
BEGIN
  INSERT INTO media_fts (rowid, title, caption, alt_text)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.caption, ''), NEW.alt_text);
END;

CREATE TRIGGER IF NOT EXISTS media_fts_update
AFTER UPDATE ON media
BEGIN
  DELETE FROM media_fts WHERE rowid = OLD.id;
  INSERT INTO media_fts (rowid, title, caption, alt_text)
  VALUES (NEW.id, NEW.title, COALESCE(NEW.caption, ''), NEW.alt_text);
END;

CREATE TRIGGER IF NOT EXISTS media_fts_delete
AFTER DELETE ON media
BEGIN
  DELETE FROM media_fts WHERE rowid = OLD.id;
END;
