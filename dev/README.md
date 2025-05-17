# ClickHouse Cluster Development Setup

This directory contains a development environment for testing substreams with a ClickHouse cluster.

## Overview

The setup includes:
- A 2-node ClickHouse cluster with ZooKeeper coordination
- Scripts for easy deployment and management
- Substreams SQL sink integration

## Prerequisites

- Docker and Docker Compose
- Substreams package (`.spkg` or `.yaml` file) either local or via URL

## Quick Start

1. **Start the ClickHouse cluster:**

   ```bash
   # Start with existing data (if any)
   ./up.sh

   # OR start with clean data directories
   ./up.sh -c
   ```

2. **Run the Substreams SQL Sink:**

   ```bash
   ./run_substreams_sql.sh
   ```

## Configuration

Edit `.env` file to customize your setup:

```bash
# ClickHouse connection
CH_USERNAME=default
CH_PASSWORD=default
CH_DATABASE=testdb
CH_HOSTNAME=localhost
CH_CLUSTER=dev1

# Substreams settings
SQL_SINK_IMAGE=ghcr.io/yaroshkvorets/substreams-sink-sql:v4.6.2-patched-ch-go
SPKG=https://github.com/pinax-network/substreams-evm-tokens/releases/download/uniswaps-v0.1.2/evm-uniswaps-v0.1.2.spkg
CURSOR_TABLE_PREFIX=backfill
START_BLOCK=20000000
STOP_BLOCK=20100000
ENDPOINT=eth.substreams.pinax.network:443
SUBSTREAMS_WORKERS=10
```

## Accessing ClickHouse

- **Node 1:**
  - HTTP interface: http://localhost:8123
  - Native interface: localhost:9000
  - Default credentials: default/default

- **Node 2:**
  - HTTP interface: http://localhost:8124
  - Native interface: localhost:9001
  - Default credentials: default/default

## Running Queries Against the Cluster

Use the cluster name in your DDL operations:

```sql
-- Create distributed table
CREATE TABLE my_table ON CLUSTER dev1 (
    id UInt64,
    name String
) ENGINE = ReplicatedMergeTree
ORDER BY id;
```

## File Structure

- `docker-compose.yaml` - Docker Compose configuration for the cluster
- `clickhouse-configs/` - Configuration files for ClickHouse nodes
- `data1/` and `data2/` - Directories used to persist data for ClickHouse nodes; mounted as volumes in the Docker Compose setup
- `up.sh` - Script to start containers (with optional data cleanup)
- `run_substreams_sql.sh` - Script to run the Substreams SQL sink
- `delete_db.sh` - Script to delete the database
- `.env` - Environment variables for configuration

## Troubleshooting

- If you encounter replication issues, try restarting with clean data:
  ```bash
  ./up.sh -c
  ```

- Check container logs for detailed error messages:
  ```bash
  docker-compose logs clickhouse01
  docker-compose logs clickhouse02
  ```
