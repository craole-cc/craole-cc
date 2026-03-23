SELECT DISTINCT kind AS "kind!: String"
FROM posts
WHERE published = 1
ORDER BY kind ASC;
