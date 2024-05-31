-- Postgres upsert (insert on conflict)
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

CREATE TABLE category (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text UNIQUE NOT NULL
);

-- Do nothing if `name` already exists
INSERT INTO category (name) VALUES ('foo'), ('foo')
ON CONFLICT (name) DO NOTHING
RETURNING id;

-- Second insert won't return id
INSERT INTO category (name) VALUES ('foo'), ('foo')
ON CONFLICT (name) DO NOTHING
RETURNING id;

-- Return id even if row isn't new
INSERT INTO category (name) VALUES ('foo'),  ('bar')
ON CONFLICT (name) DO
    UPDATE SET name = excluded.name  -- without it excluded rows would be skipped
RETURNING id;

-- Return `new = true` if the row was inserted
INSERT INTO category (name) VALUES ('foo'),  ('bar'), ('baz')
ON CONFLICT (name) DO
    UPDATE SET name = excluded.name
RETURNING id, xmax = 0 AS is_new;

SELECT * FROM category;

ROLLBACK;
