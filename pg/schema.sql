-- Change Postgres schema
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create table
CREATE TABLE person (
    id serial PRIMARY KEY,

    -- Checked column
    dob timestamptz NOT NULL DEFAULT now() CHECK(dob > '1900.01.01'),
    dod timestamptz,

    -- Table level check evaluated when any column in the table is updated
    CHECK (dod > dob)
);

-- Comments
COMMENT ON TABLE  person IS 'An example table';
COMMENT ON COLUMN person.dob IS 'Date of birth';
COMMENT ON COLUMN person.dod IS 'Date of death';

-- Add a column
ALTER TABLE person ADD COLUMN age int NOT NULL;

-- Create index
CREATE INDEX idx_age ON person (age);

-- Drop index (won't work in the same transaction as creation)
-- DROP INDEX idx_age;

-- Change a column type
ALTER TABLE person ALTER COLUMN age TYPE smallint;

-- Drop constrains
ALTER TABLE person ALTER COLUMN age DROP NOT NULL;

-- Add constrains
ALTER TABLE person ALTER COLUMN age SET NOT NULL;

-- Rename a column
ALTER TABLE person RENAME COLUMN age TO lived;

-- Delete a column
ALTER TABLE person DROP COLUMN lived;

--- Rename table
ALTER TABLE person RENAME TO people;

--- Remove table
DROP TABLE people;

ROLLBACK;
