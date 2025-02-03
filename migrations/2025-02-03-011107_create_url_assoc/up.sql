-- Your SQL goes here
ALTER TABLE url ADD COLUMN hash BIGINT NOT NULL;
CREATE INDEX hash_idx ON url(hash);
