-- Remove the original short CV seed entry.
-- The detailed profile is maintained in assets/posts/craig-craole-cole-profile.md.
DELETE FROM posts
WHERE slug = 'cv';
