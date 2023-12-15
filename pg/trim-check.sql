-- Check there's no leading or trailing whitespaces
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE person (
    -- Enaure the `name` field is trimmed
    name text CONSTRAINT name_trimmed CHECK (name = TRIM(name))
);

-- Inserting of trimmed name works
INSERT INTO person (name) VALUES ('alice');

-- ERROR:  new row for relation "person" violates check constraint "name_trimmed"                                                                                                                         
-- DETAIL:  Failing row contains ( bob).
INSERT INTO person (name) VALUES (' bob');

ROLLBACK;
