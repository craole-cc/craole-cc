-- 0007_normalize_lms_analysis_slug.sql — migrate the legacy project slug.
UPDATE projects
SET slug = 'lms-analysis'
WHERE slug = 'lms-data-analysis'
  AND NOT EXISTS (SELECT 1 FROM projects WHERE slug = 'lms-analysis');

UPDATE projects
SET screenshots = '/media/projects/lms-analysis/images/web-dashboard.png,/media/projects/lms-analysis/images/tui-dashboard.png'
WHERE slug = 'lms-analysis';
