-- Full text search
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

CREATE TABLE food (
    id serial PRIMARY KEY,
    name text NOT NULL,
    -- search vector is automatically synced with the source data (name)
    name_fts tsvector GENERATED ALWAYS AS (to_tsvector('english', name)) STORED
);

-- Index to speedup the search
CREATE INDEX ON food USING GIN (name_fts);

-- Some data to search
INSERT INTO food (name) VALUES
    ('Nut, cashew, roasted, salted'),
    ('Butter, plain, no added salt'),
    ('Plum, salted');


-- -- Search by phrases
-- \set phrase 'performant and concurrent'
-- SELECT
--     ts_rank(name_fts, websearch_to_tsquery('english', :'phrase')) as rank,
--     id,
--     name
-- FROM language
-- WHERE name_fts @@ websearch_to_tsquery(:'phrase')
-- ORDER BY rank DESC;    

-- Full text search falling back to regexp with
\set input 'salt'
SELECT
    id,
    name,
    ts_rank(to_tsvector('english', name), to_tsquery('english', :'input' || ':*')) AS rank
FROM food
WHERE to_tsvector('english', name) @@ to_tsquery(:'input' || ':*')
ORDER BY rank DESC, name ILIKE :'input' DESC;


ROLLBACK;
