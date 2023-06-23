-- Postgres enum

BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create enum
CREATE TYPE weather AS ENUM ('sunny', 'cloudy');

-- List enum values (from psql: \dT+ weather)
SELECT enum_range(NULL::weather);

-- Add a new enum value
ALTER TYPE weather ADD VALUE 'rainy';

-- Use it in a table
CREATE TABLE calendar (
    date date,
    weather weather
);
INSERT INTO calendar VALUES ('2000-01-01', 'sunny');

-- Rename the enum itself
ALTER TYPE weather RENAME TO meteo;

-- Rename an enum item
ALTER TYPE meteo RENAME VALUE 'sunny' TO 'clear';

SELECT * FROM calendar;

ROLLBACK;
