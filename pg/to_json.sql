-- Converting to json
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create initial table
CREATE TABLE lang (
    id serial PRIMARY KEY,
    name text
);

-- Insert initial data
INSERT INTO lang (name) VALUES ('Rust'), ('SQL');

\echo 'Select rows each row as a separate json object'
SELECT id, to_jsonb(lang.*) FROM lang;

\echo 'Only particular columns to to json'
SELECT json_build_object('id', id) FROM lang;

\echo 'Rows as json without some columns'
SELECT to_jsonb(lang.*) - 'id' FROM lang;
