-- Lateral join
BEGIN
;

CREATE SCHEMA sandbox;

SET
    search_path = sandbox;

CREATE TABLE person (
    id serial PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO
    person (name)
VALUES
    ('Alice'),
    ('Bob'),
    ('Carol');

SELECT
    person.*,
    -- in contrast with a subquery we can use multiple columns of the related table
    next.id AS next_id,
    next.name AS next_name
FROM
    person
    LEFT JOIN LATERAL (
        SELECT
            *
        FROM
            person next
        WHERE
            next.id > person.id -- it wouldn't be possible to reference `next.id` here without the `LATERAL`
        LIMIT
            1
    ) AS next ON TRUE;

ROLLBACK;
