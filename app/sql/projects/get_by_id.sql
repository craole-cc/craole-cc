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
    CAST(COALESCE(GROUP_CONCAT(pt.tag, ','), '') AS TEXT) AS tags
FROM projects p
LEFT JOIN project_tags pt ON pt.project_id = p.id
WHERE p.published = 1 AND p.id =
$1
GROUP BY p.id
