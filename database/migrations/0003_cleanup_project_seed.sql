-- Remove the legacy seed row whose slug used a Cyrillic character and
-- could coexist with the canonical content slug as a duplicate Dev card.
DELETE FROM project_tags
WHERE project_id IN (
  SELECT id
  FROM projects
  WHERE repo_url = 'https://github.com/craole-cc/craole-cc'
    AND slug <> 'craole-cc'
    AND EXISTS (SELECT 1 FROM projects WHERE slug = 'craole-cc')
);

DELETE FROM projects
WHERE repo_url = 'https://github.com/craole-cc/craole-cc'
  AND slug <> 'craole-cc'
  AND EXISTS (SELECT 1 FROM projects WHERE slug = 'craole-cc');

-- If the canonical content row has not yet been synced, normalize the legacy
-- row in place instead of leaving the site without its own project entry.
UPDATE projects
SET slug = 'craole-cc',
    title = 'craole.cc',
    live_url = 'https://craole.cc'
WHERE repo_url = 'https://github.com/craole-cc/craole-cc'
  AND slug <> 'craole-cc'
  AND NOT EXISTS (SELECT 1 FROM projects WHERE slug = 'craole-cc');

-- Ensure the public analysis project exists after a normal production startup,
-- even when the deployment has not separately run content sync-db yet.
INSERT INTO projects (
  title,
  slug,
  description,
  status,
  repo_url,
  live_url,
  featured,
  published,
  sort_order
)
SELECT
  'LMS Lesson Data Analysis',
  'lms-data-analysis',
  'Privacy-conscious analysis of authenticated lesson data from a real LMS workflow. The project turns an irregular operational data source into validated aggregates, period comparisons, a Ratatui terminal dashboard, a synthetic public web demo, and a roadmap toward a cross-platform app.',
  'active',
  'https://github.com/craole-cc/fluentbe',
  'https://craole.cc/data/lms',
  1,
  1,
  20
WHERE NOT EXISTS (SELECT 1 FROM projects WHERE slug = 'lms-data-analysis');

INSERT OR IGNORE INTO project_tags (project_id, tag)
SELECT id, tag
FROM projects
JOIN (
  SELECT 'LMS' AS tag UNION ALL
  SELECT 'Data Analysis' UNION ALL
  SELECT 'Analytics' UNION ALL
  SELECT 'Data Engineering' UNION ALL
  SELECT 'Rust' UNION ALL
  SELECT 'Leptos' UNION ALL
  SELECT 'Axum' UNION ALL
  SELECT 'Ratatui' UNION ALL
  SELECT 'Privacy'
) tags
WHERE projects.slug = 'lms-data-analysis';
