# Postgres FTS (full text search)

- check for supported languages: `\dF`
- search with generating vectors on-the-fly (has performance issues on large
    tables):
    ```sql
    SELECT id, text FROM documents  WHERE to_tsvector(text) @@ to_tsquery('jump & quick');
    ```
