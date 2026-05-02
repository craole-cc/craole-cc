UPDATE projects
SET readme_html =
$2
WHERE id = $1
