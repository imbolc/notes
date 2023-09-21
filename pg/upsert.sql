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
-- Returns `new = true` if the row was inserted
INSERT INTO category (id, val)
              VALUES (1, 'bar'),  (2, 'spam')
ON CONFLICT (id) DO
    UPDATE SET val = 'updated'
RETURNING xmax = 0 AS new;

SELECT * FROM category;

ROLLBACK;
