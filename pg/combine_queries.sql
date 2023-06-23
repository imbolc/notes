-- Combine two completely separate queries into a single row

WITH person (id, name) AS (VALUES
    (1, 'Alice'),
    (2, 'Bob')
),
lang (id, name) AS (VALUES
    (1, 'SQL'),
    (2, 'Rust')
)

SELECT * FROM
    (SELECT name FROM person WHERE id = 1) p,
    (SELECT name as knows FROM lang WHERE id = 2) l;

--  name  | knows                                                                                                                                  
-- -------+-------                                                                                                                                 
--  Alice | Rust 
