-- Postgres sandbox

BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;
\set ECHO all

-- Unlogged tables don't generate WAL information
CREATE UNLOGGED TABLE cache (
    key text NOT NULL,
    value text NOT NULL
);

\d cache

ROLLBACK;
