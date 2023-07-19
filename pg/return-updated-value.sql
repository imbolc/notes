-- Postgres return old value after update
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE foo (
    id serial PRIMARY KEY,
    name TEXT
);

-- Let's instert a few rows
INSERT INTO foo (name) VALUES ('Alice');

-- By default new values are returning
UPDATE foo
SET name = 'Bob'
WHERE id = 1
RETURNING name;

--  name
-- ------
--  Bob

-- To return old values we should reselect
UPDATE foo
SET name = 'Carol'
WHERE id = 1
RETURNING (SELECT name FROM foo WHERE id = 1) as old_name;

--  old_name
-- ----------
--  Alice

ROLLBACK;
