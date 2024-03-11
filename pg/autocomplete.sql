-- Autocompletion with incomplete words matching
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TABLE food (
    id serial PRIMARY KEY,
    name text NOT NULL,
    -- search vector is automatically synced with the source data (name)
    name_fts tsvector GENERATED ALWAYS AS (to_tsvector('english', name)) STORED
);

-- Index to speedup FTS
CREATE INDEX ON food USING GIN (name_fts);

-- Index to speedup trigramms 
CREATE INDEX ON food USING GIN (name gin_trgm_ops);

-- Some data to search
INSERT INTO food (name) VALUES
    ('Artichoke, globe, boiled, drained'),
    ('Oil, blend of monounsaturated vegetable oils'),
    ('Artichoke, jerusalem, peeled, boiled, drained'),
    ('Fat, solid, vegetable oil based'),
    ('Bean, haricot, dried, boiled, drained');

-- Full text search falling back to regexp with
SELECT id, name FROM food WHERE to_tsvector('english', name) @@ to_tsquery('oi' || ':*');    


ROLLBACK;
