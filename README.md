# cymer

Centralize your [e]commerce


## Commands

Because we are using relative paths, commands must be run from the root folder

- `$> bin/dev.sh`: run in dev mode
- `$> bin/seed.sh`: seed the database
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
$> ./bin/seed.sh
```


## Env

All the public variables are saved on the `env/{env}.toml` file

But the following ones are required in your env:
```
CYMER_SECRET="anythingyoulikeordontlikethatsreallyjustuptoyou"
```
! `bin/dev.sh` script automatically add a secret key