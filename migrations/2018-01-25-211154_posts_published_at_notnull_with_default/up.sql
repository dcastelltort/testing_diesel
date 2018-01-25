-- Your SQL goes here
UPDATE posts
SET publish_at = NOW()
WHERE
 publish_at IS NULL;
 
ALTER TABLE posts ALTER COLUMN publish_at SET DEFAULT NOW();
ALTER TABLE posts ALTER COLUMN publish_at SET NOT NULL;