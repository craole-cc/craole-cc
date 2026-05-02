-- Related by shared tags first, then same shoot day, then recent.
-- Excludes the current photo.
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
    CAST(COALESCE(GROUP_CONCAT(mt.tag, ','), '') AS TEXT) AS "tags!: String",
    COUNT(shared.tag) AS shared_tag_count,
    CASE WHEN DATE(m.taken_at) = DATE((
        SELECT taken_at FROM media WHERE slug = ? 1
    )) THEN 1 ELSE 0 END AS same_shoot
FROM media m
LEFT JOIN media_tags mt ON mt.media_id = m.id
LEFT JOIN media_tags shared
    ON
        shared.media_id = m.id
        AND shared.tag IN (
            SELECT tag FROM media_tags
            WHERE media_id = (SELECT id FROM media WHERE slug = ? 1)
        )
WHERE
    m.published = 1
    AND m.slug
! = ? 1
GROUP BY m.id
ORDER BY
shared_tag_count DESC,
same_shoot DESC,
m.taken_at DESC
LIMIT 6 ;
