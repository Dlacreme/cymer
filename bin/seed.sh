
DB_NAME="cymer_dev"
USER="cymer"

psql $DB_NAME $USER -h localhost < ./db/seed.sql
