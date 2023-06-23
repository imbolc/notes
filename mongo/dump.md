# Mongodb dump

```shell
mongoexport --db=dbname --collection=colname | gzip -9 > filename.json.gz
zcat filename.json.gz | mongoimport --db=dbname --collection=colname
```
