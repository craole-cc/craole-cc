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
  p.readme_html,
  p.screenshots
FROM projects p
WHERE p.published = 1 AND p.slug = $1
