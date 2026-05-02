SELECT
  p.id AS "id!: i64",
  p.title AS "title!: String",
  p.slug AS "slug!: String",
  p.excerpt AS "excerpt: String",
  p.kind AS "kind!: String",
  p.featured AS "featured!: i64",
  p.cover_url AS "cover_url: String",
  p.published_at AS "published_at: String",
  p.created_at AS "created_at!: String",
  CAST(COALESCE(GROUP_CONCAT (pt.tag, ','), '') AS TEXT) AS "tags!: String"
FROM
  posts p
  LEFT JOIN post_tags pt ON pt.post_id = p.id
WHERE
  p.published = 1
GROUP BY
  p.id
ORDER BY
  p.featured DESC,
  p.published_at DESC,
  p.created_at DESC;
