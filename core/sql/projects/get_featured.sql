-- Featured projects for the Home page — max 4.
SELECT p.id AS id,
  p.title AS title,
  p.description AS description,
  p.status AS status,
  p.repo_url AS repo_url,
  p.live_url AS live_url,
  p.featured AS featured,
  p.sort_order AS sort_order,
  p.created_at AS created_at,
  COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS tags
FROM projects p
  LEFT JOIN project_tags pt ON pt.project_id = p.id
WHERE p.published = 1
  AND p.featured = 1
GROUP BY p.id
ORDER BY p.sort_order ASC
LIMIT 4
