SELECT tag
FROM project_tags
WHERE project_id = $1
ORDER BY tag ASC
