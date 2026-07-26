-- Keep the LMS project link aligned with the public demo namespace.
UPDATE projects
SET live_url = 'https://craole.cc/demos/lms-analysis'
WHERE slug = 'lms-data-analysis';
