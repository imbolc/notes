-- Postgres aggregations
BEGIN;
CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create initial table
CREATE TABLE billing (
    id serial PRIMARY KEY,
    balance numeric(6, 2)
);

-- Aggregations with no target rows return NULL;
SELECT sum(balance)
FROM billing;

-- Returning 0 instead
SELECT coalesce(sum(balance), 0)
FROM billing;

ROLLBACK;
