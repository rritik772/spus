#!/bin/bash

# Variables
CONTAINER_NAME="spus_db"
DB_PORT="spus"
DB_PORT="5432"
DB_HOST="127.0.0.1"
DB_DATABASE="spus"
DB_PASSWORD="uUasYhasE"
DB_USER="spus"

# Check if the container already exists
if [ "$(docker ps -q -f name=$CONTAINER_NAME)" ]; then
    echo "Container '$CONTAINER_NAME' already exists and Running. Exiting..."
    exit 1
fi

docker compose -f ./docker-compose.yaml up -d

export PGPASSWORD=$DB_PASSWORD
until psql -h $DB_HOST -U $DB_USER -p $DB_PORT -d $DB_DATABASE -c '\q'; do
    >&2 echo "Postgres is still unavaliable - sleeping..."
    sleep 1
done

>&2 echo "Everything looks fine..."
