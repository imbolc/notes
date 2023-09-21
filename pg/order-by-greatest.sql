-- Order by the greatest of two fields
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE request (
    id serial PRIMARY KEY,
    success_at timestamptz,
    error_at timestamptz
);

INSERT INTO request (success_at, error_at) VALUES
    ('2000-01-02', '2000-01-03'),
    ('2000-01-01', null),
    (null, '2000-01-04'),
    (null, null)
;

SELECT * FROM request
ORDER BY GREATEST(success_at, error_at) ASC NULLS FIRST;

ROLLBACK;
