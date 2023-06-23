Wikidata
========

Working with SPARQL query builder
---------------------------------
The easy way to create a query is to use the [wikidata query builder][bulder]

Open the [builder][] type `wdt:` or `wd:` and type `Ctrl+Space` to search for
suggestions.

Basic query
-----------
Selecting all countries with their names
```sparql
SELECT ?country ?countryLabel {
  # wdt:P31  - instance of
  # wd:Q6256 - country
  # select all items where `instance_of` == `country`
  ?country wdt:P31 wd:Q6256 .

  SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
}
```

Narrowing the query
-------------------
```sparql
SELECT ?countryLabel ?population WHERE {
  ?country wdt:P31 wd:Q6256 .       # select country
  ?country wdt:P1082 ?population .  # witch population is not NULL
  SERVICE wikibase:label { bd:serviceParam wikibase:language "ru,en,[AUTO_LANGUAGE]" . }
}
ORDER BY DESC (?population)         # sorted by population
LIMIT 10                            # no more than 10 entries
```

Count
-----
```sparql
SELECT (count(*) as ?count) {
...
}
```

[builder]: https://query.wikidata.org/
