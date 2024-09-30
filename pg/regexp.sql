-- Regexp
\set ECHO all


-- split by any non-unicode character including underscore
SELECT word from regexp_split_to_table (
    'foo  (фу )bar!"#$%&''*+, -./:;<=>?@[\]^_`|~{baz_спам}',
    '[^[:alnum:]]+'
) as word
WHERE word <> '';
