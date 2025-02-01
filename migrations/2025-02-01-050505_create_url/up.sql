-- Your SQL goes here
CREATE TABLE url (
  id SERIAL PRIMARY KEY,
  original_url VARCHAR(255) NOT NULL,
  short_url VARCHAR(255) NOT NULL,
  created_on BIGINT NOT NULL,
  expiries_at BIGINT NOT NULL,
  redirection_count INTEGER NOT NULL
);
