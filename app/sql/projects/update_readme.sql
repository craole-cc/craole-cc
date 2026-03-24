-- Cache rendered README HTML for a project.
UPDATE projects
SET readme_html = ?2
WHERE id = ?1;
