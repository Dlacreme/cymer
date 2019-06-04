# cymer

Centralize your [e]commerce


## Commands

Because we are using relative paths, commands must be run from the root folder

- `$> cargo run`: run the API
- `$> cargo run ./env/prod.toml`: run the API with prod env
- `$> cargo watch -x run`: run in dev mode (live reload)
- `$> cargo watch -x test`: run test with hot reload
- `$> diesel migration run`: run the migration (up)
- `$> diesel migration redo`: reset the migration (down + up)
- `$> diesel database setup`: create the database
- `$> diesel database reset`: drop & create the database
- `$> diesel print-schema -> src/schema.rs`: build the schema.rs file


## Database

First create `cymer` user with approprivate access:
```
$> sudo -u postgres psql
** connect to your local psql
postgres=# CREATE USER cymer WITH ENCRYPTED PASSWORD 'cymer_rules';
postgres=# ALTER USER cymer WITH CREATEDB;
postgres=# \q
```

Install `diesel-cli`  ( https://github.com/diesel-rs/diesel/tree/master/diesel_cli )

Run:
```
$> diesel database setup
$> diesel migration run
$> ./db/seed.sh
```
