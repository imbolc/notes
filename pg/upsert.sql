-- Postgres upsert (insert on conflict)
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE category (
    id int PRIMARY KEY,
    val text NOT NULL
);

-- Create an initial value
INSERT INTO category VALUES (1, 'foo');

-- Do nothing if `id` already exists
INSERT INTO category VALUES (1, 'bar')
ON CONFLICT (id) DO NOTHING;

-- Update value if `id` exists
INSERT INTO category (id, val)
              VALUES (1, 'bar')
ON CONFLICT (id) DO
    UPDATE SET val = 'baz';

SELECT * FROM category;

ROLLBACK;
