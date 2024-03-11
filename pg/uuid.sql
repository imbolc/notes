-- Postgres UUID
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Using UUID as a primary key
CREATE TABLE foo (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid()
);

INSERT INTO foo DEFAULT VALUES;
SELECT * FROM foo;

ROLLBACK;
