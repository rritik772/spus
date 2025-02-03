-- This file should undo anything in `up.sql`

DROP INDEX hash_idx;
ALTER TABLE url DROP COLUMN hash;
