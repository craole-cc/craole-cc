SELECT
  p.id,
  p.title,
  p.slug,
  p.description,
  p.status,
  p.repo_url,
  p.live_url,
  p.featured,
  p.sort_order,
  p.created_at,
  (
    SELECT COALESCE(GROUP_CONCAT(pt.tag, ','), '')
    FROM project_tags AS pt
    WHERE pt.project_id = p.id
  ) AS tags
FROM projects AS p
WHERE p.published = 1
  AND p.id IN (
    SELECT project_id
    FROM project_tags
    WHERE tag = $1
  )
ORDER BY p.sort_order ASC, p.created_at DESC;
