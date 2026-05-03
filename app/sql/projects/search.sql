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
  COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS tags
FROM
  projects AS p
  LEFT JOIN project_tags AS pt ON pt.project_id = p.id
WHERE
  p.published = 1
  AND p.id IN (
    SELECT
      rowid
    FROM
      projects_fts
    WHERE
    where
      projects_fts MATCH ?
  )
GROUP BY
  p.id,
  p.title,
  p.slug,
  p.description,
  p.status,
  p.repo_url,
  p.live_url,
  p.featured,
  p.sort_order,
  p.created_at
ORDER BY
  p.sort_order ASC;
