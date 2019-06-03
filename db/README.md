# niplate Database

## See or update Schema

1. use `https://ondras.zarovi.cz/sql/demo/?keyword=default` to see and update the schema (wwwschema.xml)
2. use the ondras website to generate a psql dump
3. run `bin/db_setup.sh` to create the db or `bin/db_reset.sh` to update the model
4. Enjoy

Creating the psql dump from ondras requires to switch few types:
```
`datetime` -> `timestamp`
`tinyint` -> `bool`
```
