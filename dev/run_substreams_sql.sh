#!/bin/bash

set -e

# Configurable variables
CH_USERNAME="${CH_USERNAME:-default}"
CH_PASSWORD="${CH_PASSWORD:-default}"
CH_DATABASE="${CH_DATABASE:-testdb}"
CH_HOSTNAME="${CH_HOSTNAME:-localhost}"
CH_CLUSTER="${CH_CLUSTER:-default}"
SQL_SINK_IMAGE="${SQL_SINK_IMAGE:-ghcr.io/yaroshkvorets/substreams-sink-sql:v4.6.2-patched-ch-go}"
SPKG="${SPKG:-https://github.com/pinax-network/substreams-evm-tokens/releases/download/uniswaps-v0.1.2/evm-uniswaps-v0.1.2.spkg}"
CURSOR_TABLE_PREFIX="${CUSOR_TABLE_PREFIX:-backfill}"
START_BLOCK="${START_BLOCK:-10000000}"
STOP_BLOCK="${STOP_BLOCK:-10100000}"
ENDPOINT="${ENDPOINT:-https://eth.substreams.pinax.network:443}"
SUBSTREAMS_WORKERS="${SUBSTREAMS_WORKERS:10}"
OTHER_ARGS="${OTHER_ARGS:-}"
SUBSTREAMS_API_TOKEN="${SUBSTREAMS_API_TOKEN:-}"

# Wait for ClickHouse to be ready
until docker exec clickhouse clickhouse-client --user $CH_USERNAME --password $CH_PASSWORD --query "SELECT 1" &>/dev/null; do
  echo "Waiting for ClickHouse to be ready..."
  sleep 2
done

echo "ClickHouse is ready. Creating database if not exists..."
docker exec clickhouse clickhouse-client --user $CH_USERNAME --password $CH_PASSWORD --query "CREATE DATABASE IF NOT EXISTS $CH_DATABASE;"

echo "Running substreams-sink-sql setup..."
docker run --rm \
  --network host \
  -v $(pwd):/app \
  ${SQL_SINK_IMAGE} \
  /app/substreams-sink-sql setup \
    clickhouse://$CH_USERNAME:$CH_PASSWORD@$CH_HOSTNAME:9000/$CH_DATABASE \
    ${SPKG} \
    --cursors-table=${CURSOR_TABLE_PREFIX}_${START_BLOCK} \
    --ignore-duplicate-table-errors
    # --clickhouse-cluster $CH_CLUSTER \

# Run the substreams sink

echo "Running substreams-sink-sql run..."
docker run --rm \
  --network host \
  -e SUBSTREAMS_API_TOKEN=$SUBSTREAMS_API_TOKEN \
  -v $(pwd):/app \
  ${SQL_SINK_IMAGE} \
  /app/substreams-sink-sql run \
    clickhouse://$CH_USERNAME:$CH_PASSWORD@$CH_HOSTNAME:9000/$CH_DATABASE \
    ${SPKG} \
    ${START_BLOCK}:${STOP_BLOCK} \
    --endpoint=$ENDPOINT \
    --cursors-table=${CURSOR_TABLE_PREFIX}_${START_BLOCK} \
    -H "X-Sf-Substreams-Parallel-Jobs: $SUBSTREAMS_WORKERS" \
    --batch-row-flush-interval 100000 \
    --batch-block-flush-interval 1000 \
    --metrics-listen-addr 0.0.0.0:9102 \
    --delay-before-start=5s \
    --final-blocks-only \
    --on-module-hash-mistmatch error \
    --undo-buffer-size 1 \
    ${OTHER_ARGS}
    # --clickhouse-cluster $CH_CLUSTER \
