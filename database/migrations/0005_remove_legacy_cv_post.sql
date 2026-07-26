-- Remove the original short CV seed entry.
-- The detailed profile is maintained in content/posts/craig-craole-cole-profile.md.
DELETE FROM posts
WHERE slug = 'cv';
