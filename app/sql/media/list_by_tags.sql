-- Filter media that has ALL of the given tags (intersection / AND logic).
-- ?1 = newline-separated tag list, e.g. "Landscape\nJamaica"
-- Split with a recursive CTE, then count matches per media item.
-- Only keep items where every requested tag matched.

WITH RECURSIVE split (tag, rest) AS (
    SELECT
        TRIM(substr (? 1, 1, INSTR (? 1 | | char (10), char (10)) - 1)),
        TRIM(substr (? 1, INSTR (? 1 | | char (10), char (10)) + 1))
    UNION ALL
    SELECT
        TRIM(SUBSTR(rest, 1, INSTR(rest || CHAR(10), CHAR(10)) - 1)),
        TRIM(SUBSTR(rest, INSTR(rest || CHAR(10), CHAR(10)) + 1))
    FROM split
    WHERE rest != ''
),

requested AS (
    SELECT tag FROM split WHERE tag != ''
),

tag_count AS (
    SELECT COUNT(*) AS cnt FROM requested
)

SELECT
    m.id AS "id!: i64",
    m.title AS "title!: String",
    m.slug AS "slug!: String",
    m.caption AS "caption: String",
    m.media_type AS "media_type!: String",
    m.file_path AS "file_path!: String",
    m.alt_text AS "alt_text!: String",
    m.width AS "width: i64",
    m.height AS "height: i64",
    m.sort_order AS "sort_order!: i64",
    m.taken_at AS "taken_at: String",
    m.created_at AS "created_at!: String",
    CAST(COALESCE(GROUP_CONCAT(mt2.tag, ','), '') AS TEXT) AS "tags!: String"
FROM media m
JOIN media_tags mt ON mt.media_id = m.id
JOIN requested r ON mt.tag = r.tag
LEFT JOIN media_tags mt2 ON mt2.media_id = m.id
WHERE m.published = 1
GROUP BY m.id
HAVING COUNT(DISTINCT mt.tag) = (SELECT cnt FROM tag_count)
ORDER BY m.sort_order ASC, m.taken_at DESC;
