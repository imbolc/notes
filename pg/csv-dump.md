# Postgres csv dump

```sql
\copy (SELECT * FROM my_table) to '/path/to/file.csv' with csv;
```
