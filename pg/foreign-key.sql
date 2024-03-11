-- Foreign key
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

CREATE TABLE db_type (
    id serial PRIMARY KEY,
    name text UNIQUE NOT NULL
);

CREATE TABLE db (
    id serial PRIMARY KEY,
    name text UNIQUE NOT NULL,
    type_id int NOT NULL REFERENCES db_type(id) ON DELETE CASCADE
);

-- It's a common best practice to create indexes on foreign key columns, for speed-up joining
CREATE INDEX ON db(type_id);

INSERT INTO db_type (name) VALUES ('relational'), ('document');
INSERT INTO db (name, type_id) VALUES ('posgres', 1), ('mongo', 2);

SELECT
    d.name,
    t.name as kind
FROM db d
JOIN db_type t ON d.type_id = t.id;

--   name   |    kind
-- ---------+------------
--  posgres | relational
--  mongo   | document

ROLLBACK;
