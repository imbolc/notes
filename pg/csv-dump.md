# Postgres csv dump

```sql
\copy (table my_table) to './my_table.csv' CSV;
```

Copy table in `tsv` format with header:

```sql
\copy (SELECT * FROM my_table ORDER BY id) to './my_table.tsv' DELIMITER '\t' CSV HEADER;
```
