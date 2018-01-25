-- This file should undo anything in `up.sql`
ALTER TABLE posts ALTER COLUMN publish_at DROP DEFAULT;
ALTER TABLE posts ALTER COLUMN publish_at DROP NOT NULL;