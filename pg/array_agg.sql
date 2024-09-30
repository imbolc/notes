-- Postgres array_agg aggregation
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

-- Create initial table
CREATE TABLE person (
    id serial PRIMARY KEY,
    name text
);

-- This will return `null` if the array is empty
SELECT array_agg(id) FROM person;

-- Returning empty array instead of `null` if no elements is selected
SELECT coalesce(array_agg(id), '{}') FROM person;

-- Another option of returning an empty array
SELECT ARRAY(SELECT id FROM person);

INSERT INTO person (name) VALUES ('foo'), ('bar'), ('foo');

-- Unique values
SELECT array_agg(DISTINCT name) FROM person;

ROLLBACK;
