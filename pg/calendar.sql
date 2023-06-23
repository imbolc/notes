-- Calendar
BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

SELECT 
    ts::date,
    to_char(ts, 'Dy') as dow  -- day of week
FROM generate_series(         -- generate days for March of 2000
    date '2000-02-01',
    date '2000-03-01' - interval '1 day',
    interval '1 day'
) ts;

ROLLBACK;
