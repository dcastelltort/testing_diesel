-- Your SQL goes here
UPDATE posts
SET visit_count = 0
WHERE
 visit_count IS NULL;
 
ALTER TABLE posts ALTER COLUMN visit_count SET DEFAULT 0;
ALTER TABLE posts ALTER COLUMN visit_count SET NOT NULL;