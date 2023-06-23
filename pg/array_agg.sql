-- Postgres array_agg aggregation
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create initial table
CREATE TABLE person (
    id serial PRIMARY KEY
);

-- Returning emply array instead of `null` if no elements is selected
SELECT coalesce(array_agg(id), '{}') FROM person;

-- Another option of returning an empty array
SELECT ARRAY(SELECT id FROM person);

INSERT INTO person (id) VALUES (1), (2);

-- This will return `null` if the array is empty
SELECT array_agg(id) FROM person;

ROLLBACK;
