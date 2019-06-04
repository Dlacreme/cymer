
DB_NAME="cymer_dev"
USER="cymer"

psql $DB_NAME $USER < ./db/seed.psql
