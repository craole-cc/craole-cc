SELECT p.id          AS "id!: i64",
       p.title       AS "title!: String",
       p.description AS "description!: String",
       p.status      AS "status!: String",
       p.repo_url    AS "repo_url: String",
       p.live_url    AS "live_url: String",
       p.featured    AS "featured!: i64",
       p.sort_order  AS "sort_order!: i64",
       p.created_at  AS "created_at!: String",
       CAST(COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS TEXT) AS "tags!: String"
FROM projects p
  JOIN projects_fts ON projects_fts.rowid = p.id
  LEFT JOIN project_tags pt ON pt.project_id = p.id
WHERE projects_fts MATCH ?1
  AND p.published = 1
GROUP BY p.id
ORDER BY rank;
