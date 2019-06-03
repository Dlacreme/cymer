#!/usr/bin/env bash

DB_NAME="cymer_dev"
USER="cymer"
PASSWORD="cymer_rules"

if [ "$1" = "init" ]
then
  sudo -u postgres psql -c "CREATE DATABASE "$DB_NAME;
  sudo -u postgres psql -c "CREATE USER "$USER" WITH ENCRYPTED PASSWORD '"$PASSWORD"'";
  sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE "$DB_NAME" TO "$USER;
  psql $DB_NAME $USER < ./db/import.psql
  psql $DB_NAME $USER < ./db/seed.psql
else
  sudo -u postgres psql -c "DROP DATABASE "$DB_NAME;
  sudo -u postgres psql -c "CREATE DATABASE "$DB_NAME;
  sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE "$DB_NAME" TO "$USER;
  psql $DB_NAME $USER < ./db/import.psql
  psql $DB_NAME $USER < ./db/seed.psql
fi
