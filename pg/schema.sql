-- Change Postgres schema
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

-- Create table
CREATE TABLE person (
    id int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,

    -- Trimmed text column
    name text CONSTRAINT name_trimmed CHECK (name = TRIM(name)),

    -- Constrains
    dob timestamptz NOT NULL DEFAULT now() 
        CONSTRAINT dob_min CHECK(dob > '1900.01.01')
        CONSTRAINT dob_max CHECK(dob < '2000.01.01'),

    dod timestamptz,

    -- Table level check evaluated when any column in the table is updated
    CONSTRAINT born_before_death CHECK (dod > dob)
);

-- Comments
COMMENT ON TABLE  person IS 'An example table';
COMMENT ON COLUMN person.dob IS 'Date of birth';

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
ALTER TABLE person ADD CONSTRAINT age_positive CHECK (age > 0);

-- Rename a column
ALTER TABLE person RENAME COLUMN age TO lived;

-- Delete a column
ALTER TABLE person DROP COLUMN lived;

--- Rename table
ALTER TABLE person RENAME TO people;

--- Remove table
DROP TABLE people;

ROLLBACK;
