# cymer

Centralize your [e]commerce


## Commands

Because we are using relative paths, commands must be run from the root folder

- `$> ./bin/db.sh init`: create and set up the database
- `$> ./bin/db.sh`: reset the database
- `$> cargo run`: run the API
- `$> cargo run ./env/prod.toml`: run the API with prod env
- `$> cargo watch -x run`: run in dev mode (live reload)
- `$> cargo watch -x test`: run test with hot reload


## Database

 * `/db` folder contains a schema usable with `https://ondras.zarovi.cz/sql/demo/?keyword=default`
 * `/db/seed.psql` contains the DB seed

Update Schema:
 1. Go on ondras application and update the schema as you wish
 2. Save as xml your new schema and overwrite the existing one
 3. Extract your new schema as sql and overwrite `db/import.psql`
 4. Ondras has some issue with psql types. Therefore, make sure you replace:
 ```
`datetime` -> `timestamp`
`tinyint` -> `bool`
```
