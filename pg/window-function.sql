-- Window functions

-- Window function is similar to aggregation used with `GROUP BY`,
-- but instead of grouping rows it adds a column to each row.
-- They apply to the result of JOIN, WHERE, GROUP BY and HAVING clause.

BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE lang_type (
    id serial PRIMARY KEY,
    name text UNIQUE NOT NULL
);

CREATE TABLE lang (
    id serial PRIMARY KEY,
    name text UNIQUE NOT NULL,
    github_stars int NOT NULL,
    type_id int REFERENCES lang_type(id) ON DELETE CASCADE
);

INSERT INTO lang_type (name) VALUES ('compiled'), ('interprited');
INSERT INTO lang (name, github_stars, type_id) VALUES ('Go', 95300, 1), ('Lua', 5500, 2), ('Python', 45000, 2), ('Rust', 66000, 1);

SELECT
    l.name,
    t.name as kind,
    l.github_stars as stars,
    avg(l.github_stars) OVER (PARTITION BY t.name) as avg_stars_by_type
FROM lang l
JOIN lang_type t ON l.type_id = t.id;


--   name  |    kind     | stars | avg_stars_by_type
-- --------+-------------+-------+--------------------
--  Go     | compiled    | 95300 | 80650.000000000000
--  Rust   | compiled    | 66000 | 80650.000000000000
--  Lua    | interprited |  5500 | 25250.000000000000
--  Python | interprited | 45000 | 25250.000000000000

ROLLBACK;
