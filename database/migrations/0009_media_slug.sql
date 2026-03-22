ALTER TABLE media ADD COLUMN slug TEXT NOT NULL DEFAULT '';

UPDATE media SET slug = 'placeholder' WHERE slug = '';

CREATE UNIQUE INDEX IF NOT EXISTS media_slug_idx ON media(slug);
