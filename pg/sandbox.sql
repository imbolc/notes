-- Postgres sandbox

BEGIN;                      -- start a transaction
CREATE SCHEMA sandbox;      -- create a new schema
SET search_path = sandbox;  -- and make it default
-- print queries and comments
\set ECHO all

CREATE TABLE foo (id int);  -- play with something

ROLLBACK;                   -- roll everything back
