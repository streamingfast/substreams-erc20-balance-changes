#!/bin/bash

set -e

# Script to start ClickHouse cluster and optionally clean data directories
# Usage: ./up.sh [-c] (use -c flag to clean data directories)

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if -c flag is used
CLEAN=false
while getopts "c" opt; do
  case $opt in
    c)
      CLEAN=true
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

# Stop any running containers
echo "📥 Stopping any running containers..."
docker-compose down

# Clean data directories if -c flag is used
if [ "$CLEAN" = true ]; then
  echo "🧹 Cleaning data directories..."
  rm -rf data1 data2 data
  mkdir data1 data2
  echo "✅ Data directories cleaned"
fi

# Start the containers
echo "🚀 Starting ClickHouse cluster..."
docker-compose -p clickhouse-cluster up -d

# Wait for ClickHouse to be ready
echo "⏳ Waiting for ClickHouse to be ready..."
until docker exec clickhouse01 clickhouse-client --user default --password default --query "SELECT 1" &>/dev/null; do
  echo "."
  sleep 2
done

echo "✅ ClickHouse cluster is ready!"
echo
echo "You can access:"
echo "- ClickHouse 1: http://localhost:8123/play (HTTP) or localhost:9000 (Native)"
echo "- ClickHouse 2: http://localhost:8124/play (HTTP) or localhost:9001 (Native)"
echo
echo "To run the substreams-sink-sql, use:"
echo "  ./run_substreams_sql.sh"
