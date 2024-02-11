-- Full text search
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

CREATE TABLE person (
    id serial PRIMARY KEY,
    name text NOT NULL,
    -- search vector is automatically synced with the source data (name)
    name_fts tsvector GENERATED ALWAYS AS (to_tsvector('english', name)) STORED
);

-- Index to speedup the search
CREATE INDEX name_fts_idx ON person USING GIN (name_fts);

-- Some data to search
INSERT INTO person (name) VALUES
    ('Rust is a multi-paradigm, general-purpose programming language for performance, type safety, and concurrency.'),
    ('SQL is a domain-specific language used to manage data, especially in a relational database management system.');

-- Whole word search
SELECT id, name
FROM person
WHERE name_fts @@ to_tsquery('type');    

-- Include incomplete words
SELECT id, name
FROM person
WHERE name_fts @@ to_tsquery('lang' || ':*');    

ROLLBACK;
