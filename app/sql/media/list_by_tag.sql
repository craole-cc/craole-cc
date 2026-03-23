SELECT
  m.id         AS "id!: i64",
  m.title      AS "title!: String",
  m.slug       AS "slug!: String",
  m.caption    AS "caption: String",
  m.media_type AS "media_type!: String",
  m.file_path  AS "file_path!: String",
  m.alt_text   AS "alt_text!: String",
  m.width      AS "width: i64",
  m.height     AS "height: i64",
  m.sort_order AS "sort_order!: i64",
  m.taken_at   AS "taken_at: String",
  m.created_at AS "created_at!: String",
  CAST(COALESCE(GROUP_CONCAT(mt.tag, ','), '') AS TEXT) AS "tags!: String"
FROM media m
  JOIN media_tags mt ON mt.media_id = m.id
WHERE m.published = 1
  AND mt.tag = ?1
GROUP BY m.id
ORDER BY m.sort_order ASC, m.taken_at DESC;
