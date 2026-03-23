SELECT DISTINCT pt.tag AS "tag!: String"
FROM project_tags pt
  JOIN projects p ON p.id = pt.project_id
WHERE p.published = 1
ORDER BY pt.tag ASC;
