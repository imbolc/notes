-- Using SQL WITH

-- Create and use a "temporary table" in a single query
-- it could be helpful for examples of Rust sqlx macros

WITH person (id, name) AS (VALUES
    (1, 'Alice'),
    (2, 'Bob')
)
SELECT * FROM person;


-- Multiple WITH
WITH
    person (id, name) AS (VALUES
        (1, 'Alice'),
        (2, 'Bob')
    ),
    lang (person_id, name) AS (VALUES
        (1, 'SQL'),
        (1, 'Rust')
    )
SELECT
    p.name,
    l.name as knows
    FROM lang l
    RIGHT JOIN person p ON l.person_id = p.id;
