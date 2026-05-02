SELECT
  tag
FROM
  project_tags
WHERE
  project_id = ?
ORDER BY
  tag ASC
