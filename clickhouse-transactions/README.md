# EVM Transactions: `Clickhouse`

## Quickstart

**Install [`substreams-sink-sql`](https://github.com/streamingfast/substreams-sink-sql)**

```bash
brew install streamingfast/tap/substreams-sink-sql
```

### Setup SQL tables in Clickhouse

```bash
substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml
```

### Load Clickhouse data from Substreams

```bash
substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml \
    -e eth.substreams.pinax.network:443 21525891:
```

### Perform SQL query with Clickhouse

```sql
-- Select all transactions from block 123456
SELECT
    *
FROM transactions
WHERE block_num = 123456;
```
