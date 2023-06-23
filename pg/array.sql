-- Postgres array
\set ECHO queries

-- Literals
SELECT ARRAY[1, 2];  -- array of integers
SELECT '{1, 2}'::int[];  -- without explicit casting would be just a string
SELECT '{1, 2}' = ARRAY[1, 2]; -- true

-- Check if array is empty 
SELECT '{}'::int[] = '{}'; -- true

-- Check if array contains an item
SELECT 1 = ANY('{1, 2}'); -- true

-- Check if array doesn't contain an item
SELECT NOT 1 = ANY('{1, 2}'); -- false

-- Remove elements e.g. nulls from an array
SELECT array_remove('{1, null, 2}', NULL); -- {1, 2}

-- Convert array into rows
SELECT unnest('{1, 2}'::int[]);
