-- This file should undo anything in `up.sql`
ALTER TABLE posts ALTER COLUMN visit_count DROP DEFAULT;
ALTER TABLE posts ALTER COLUMN visit_count DROP NOT NULL;