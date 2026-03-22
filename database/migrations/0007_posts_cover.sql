ALTER TABLE posts ADD COLUMN cover_url TEXT;

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1511379938547-c1f69419868d?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'cv';

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1510915361894-db8b60106cb1?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'music-and-code';

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'portfolio-in-rust';

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1547036967-23d11aacaee0?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'nixos-declarative-infrastructure';

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1507838153414-b4b713384a76?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'teaching-english-rhythm';

UPDATE posts SET cover_url = 'https://images.unsplash.com/photo-1593720213428-28a5b9e94613?auto=format&fit=crop&w=1200&q=80'
WHERE slug = 'why-leptos-over-nextjs';
