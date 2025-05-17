#!/bin/bash

# Load config from .env if present
if [ -f "$(dirname "$0")/.env" ]; then
  set -o allexport
  source "$(dirname "$0")/.env"
  set +o allexport
fi


read -p "Are you sure you want to delete the ClickHouse database '$CH_DATABASE'? This action cannot be undone! (y/N): " confirm
if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
  echo "Aborted."
  exit 1
fi

echo "Dropping ClickHouse database '$CH_DATABASE'..."
docker exec clickhouse clickhouse-client --user $CH_USERNAME --password $CH_PASSWORD --query "DROP DATABASE IF EXISTS $CH_DATABASE;"

if [ $? -eq 0 ]; then
  echo "Database '$CH_DATABASE' dropped successfully."
else
  echo "Failed to drop database '$CH_DATABASE'."
  exit 1
fi
