#!/bin/bash

set -e

# Load config from .env if present
if [ -f "$(dirname "$0")/.env" ]; then
  set -o allexport
  source "$(dirname "$0")/.env"
  set +o allexport
fi


# Wait for ClickHouse to be ready
until docker exec clickhouse01 clickhouse-client --user $CH_USERNAME --password $CH_PASSWORD --query "SELECT 1" &>/dev/null; do
  echo "Waiting for ClickHouse to be ready..."
  sleep 2
done

echo "⛏️ ClickHouse is ready. Creating database if not exists..."
docker exec clickhouse01 clickhouse-client --user $CH_USERNAME --password $CH_PASSWORD --query "CREATE DATABASE IF NOT EXISTS $CH_DATABASE ${CH_CLUSTER:+ON CLUSTER $CH_CLUSTER};"

echo "🏗️ Running substreams-sink-sql setup..."
# Determine if SPKG is a local file or a URL
if [[ "$SPKG" =~ ^https?:// ]]; then
  # Remote SPKG - no mount needed
  MOUNT_ARGS=""
  SPKG_ARG="$SPKG"
else
  # Local SPKG - mount it into the container
  MOUNT_ARGS="-v $(realpath "$SPKG"):/spkg.spkg"
  SPKG_ARG="/spkg.spkg"
fi

docker run --rm \
  --network host \
  $MOUNT_ARGS \
  ${SQL_SINK_IMAGE} \
  setup \
    clickhouse://$CH_USERNAME:$CH_PASSWORD@$CH_HOSTNAME:9000/$CH_DATABASE \
    $SPKG_ARG \
    --cursors-table=${CURSOR_TABLE_PREFIX}_${START_BLOCK} \
    --ignore-duplicate-table-errors \
    ${CH_CLUSTER:+--clickhouse-cluster $CH_CLUSTER}

# Run the substreams sink

echo "🏃‍♂️ Running substreams-sink-sql run..."
# Determine if SPKG is a local file or a URL
if [[ "$SPKG" =~ ^https?:// ]]; then
  # Remote SPKG - no mount needed
  MOUNT_ARGS=""
  SPKG_ARG="$SPKG"
else
  # Local SPKG - mount it into the container
  MOUNT_ARGS="-v $(realpath "$SPKG"):/spkg.spkg"
  SPKG_ARG="/spkg.spkg"
fi

docker run --rm \
  --network host \
  -e SUBSTREAMS_API_TOKEN=$SUBSTREAMS_API_TOKEN \
  $MOUNT_ARGS \
  ${SQL_SINK_IMAGE} \
  run \
    clickhouse://$CH_USERNAME:$CH_PASSWORD@$CH_HOSTNAME:9000/$CH_DATABASE \
    $SPKG_ARG \
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
    ${OTHER_ARGS} \
    ${CH_CLUSTER:+--clickhouse-cluster $CH_CLUSTER}
