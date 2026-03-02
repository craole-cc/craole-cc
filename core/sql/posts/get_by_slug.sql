-- -- Full post including Markdown body — only called for detail page.
-- -- CAST(... AS TEXT) forces sqlx to treat the aggregated tags column
-- -- as TEXT rather than NULL (SQLite's default for GROUP_CONCAT when
-- -- analysed against a bound parameter).
-- SELECT p.id AS id,
--   p.title AS title,
--   p.slug AS slug,
--   p.body AS body,
--   p.excerpt AS excerpt,
--   p.kind AS kind,
--   p.published_at AS published_at,
--   p.created_at AS created_at,
--   CAST(COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS TEXT) AS tags
-- FROM posts p
--   LEFT JOIN post_tags pt ON pt.post_id = p.id
-- WHERE p.slug = $1
--   AND p.published = 1
-- GROUP BY p.id
SELECT p.id AS "id!: i64",
  p.title AS "title!: String",
  p.slug AS "slug!: String",
  p.body AS "body!: String",
  p.excerpt AS "excerpt: String",
  p.kind AS "kind!: String",
  p.published_at AS "published_at: String",
  p.created_at AS "created_at!: String",
  CAST(COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS TEXT) AS "tags!: String"
FROM posts p
  LEFT JOIN post_tags pt ON pt.post_id = p.id
WHERE p.slug = ?1
  AND p.published = 1
GROUP BY p.id;
