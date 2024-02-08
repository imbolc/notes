-- Ordering
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create table
CREATE TABLE items (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    at TIMESTAMPTZ
);

INSERT INTO items (at) VALUES
    (null),
    (now()),
    (null),
    (now() - interval '1 hour');

-- By default with ascending order nulls go last
SELECT * FROM items ORDER BY at;

-- Make them go first
SELECT * FROM items ORDER BY at NULLS FIRST;


ROLLBACK;
