# Postgres csv dump

```sql
\copy (table my_table) TO './my_table.csv' CSV;
```

Copy table in `tsv` format with header:

```sql
\copy (SELECT * FROM my_table ORDER BY id) TO './my_table.tsv' DELIMITER E'\t' CSV HEADER;
```
