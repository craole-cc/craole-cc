-- Single published media item — for lightbox / detail view.
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
WHERE id = $1
  AND published = 1
