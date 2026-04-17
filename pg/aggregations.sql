-- Postgres aggregations
CREATE TABLE items (name text, price int, discount float);

SELECT
    sum(price),  -- aggregations with no target rows return NULL
    coalesce(sum(price), 0) -- a way to handle this
FROM
    items;

INSERT INTO
    items (name, price, discount)
VALUES
    ('foo', 1, NULL),
    ('bar', 2, NULL),
    ('foo', 2, NULL),
    ('baz', 4, NULL);

SELECT
    name,
    count(*) AS num_items,  -- aggregate: count items
    avg(price) AS avg_price,  -- aggregate: average price
    coalesce(sum(discount), 0) AS total_discount -- without coalesce it would return null
FROM
    items
WHERE
    price IS NOT NULL -- filter rows before grouping
GROUP BY
    name -- group items with the same name
HAVING
    sum(price) >= 3 -- filter rows after grouping
ORDER BY
    avg_price DESC -- order by an aggregated value
;
