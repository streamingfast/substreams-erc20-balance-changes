# EVM Tokens: `Clickhouse`

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
-- Select the top 10 addresses with the most USDT
SELECT
    owner AS address,
    new_balance AS amount,
    date
FROM balances FINAL
WHERE contract = '0xdac17f958d2ee523a2206206994597c13d831ec7' AND new_balance > 0
ORDER BY amount DESC
LIMIT 10
```
