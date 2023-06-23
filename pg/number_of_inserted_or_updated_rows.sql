-- Postgres get number of inserted or updated rows
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE foo ( id serial PRIMARY KEY );

-- Let's instert a few rows
INSERT INTO foo (id) VALUES (1), (2);

WITH rows AS (
    INSERT INTO foo (id)
    VALUES
        (1), -- this insert will fail
        (3)
    ON CONFLICT DO NOTHING
    RETURNING 1
)
SELECT count(*) FROM rows;

--  count                                                                                                                                          
-- -------                                                                                                                                         
--      1

WITH rows AS (
    UPDATE foo
    SET id = id + 10
    RETURNING 1
)
SELECT count(*) FROM rows;

--  count                                                                                                                                          
-- -------                                                                                                                                         
--      3


ROLLBACK;
