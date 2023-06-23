-- SQL grouing

-- Grouping allows combining multile lines into one using aggregate funcitons

BEGIN; CREATE SCHEMA sandbox; SET search_path = sandbox;

CREATE TABLE items (
    name text,
    price int
);

INSERT INTO items (name, price)
VALUES ('foo', 1), ('bar', 2), ('foo', 2), ('baz', 4);

SELECT
    name,
    count(*) as num_items,      -- aggregate: count items
    sum(price) as total_price,  -- aggregate: sum prices
    avg(price) as avg_price     -- aggregate: average price
FROM items
GROUP BY name                   -- group items with the same name
HAVING sum(price) >= 3          -- filter using aggregation
ORDER BY total_price DESC;      -- order by an aggregated value

ROLLBACK;
