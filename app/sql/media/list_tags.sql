SELECT DISTINCT tag AS "tag!: String"
FROM media_tags
WHERE media_id IN (SELECT id FROM media WHERE published = 1)
ORDER BY tag ASC;
