-- Distinct tags across all published projects, sorted alphabetically.
SELECT DISTINCT tag AS "tag!: String"
FROM project_tags
WHERE project_id IN (SELECT id FROM projects WHERE published = 1)
ORDER BY tag ASC;
