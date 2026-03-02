-- All published media, ordered for gallery display.
-- Filter by media_type in Rust after fetch — avoids duplicate query variants.
SELECT id AS id,
  title AS title,
  caption AS caption,
  media_type AS media_type,
  file_path AS file_path,
  alt_text AS alt_text,
  width AS width,
  height AS height,
  sort_order AS sort_order,
  taken_at AS taken_at,
  created_at AS created_at
FROM media
WHERE published = 1
ORDER BY sort_order ASC,
  taken_at DESC,
  created_at DESC
