# ERC-20: `Clickhouse`

## Quickstart

**Install [`substreams-sink-sql`](https://github.com/streamingfast/substreams-sink-sql)**

```bash
brew install streamingfast/tap/substreams-sink-sql
```

### Setup SQL tables in Clickhouse

```bash
substreams-sink-sql setup clickhouse://default:default@localhost:9000/default \
    https://spkg.io/pinax-network/erc20-balances-v1.5.0.spkg
```

### Load Clickhouse data from Substreams

```bash
substreams-sink-sql run clickhouse://default:default@localhost:9000/default \
    https://spkg.io/pinax-network/erc20-balances-v1.5.0.spkg \
     -e eth.substreams.pinax.network:443 21525891:
```

### Perform SQL query with Clickhouse

```sql
-- Select the top sending addresses for DAI by total transferred value.
SELECT
    "from",
    count() as total_transfers,
    sum(value::HUGEINT / 10**18)::DECIMAL(18, 2) as total_value
FROM read_parquet('./out/transfers/*.parquet') AS t
WHERE contract = '6b175474e89094c44da98b954eedeac495271d0f' -- DAI
GROUP BY "from" ORDER BY total_value DESC LIMIT 30;
```
