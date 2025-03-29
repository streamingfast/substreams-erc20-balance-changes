# EVM Tokens: `ERC20 Balances & Transfers`

> Substreams for tracking ERC-20 token balances & transfers for EVM blockchains.

## Algorithms

- [x] Extract ERC-20 transfers via `logs_with_calls` events.
- [x] Extract ERC-20 balances via `storage_changes` within a call or child call.
- [ ] TO-DO: Extract ERC-20 rebasing tokens

## Quickstart

### Install [`substreams-sink-files`](https://github.com/streamingfast/substreams-sink-files)

```bash
brew install streamingfast/tap/substreams-sink-files
```

### Install [Duckdb](https://duckdb.org/#quickinstall)

```bash
curl https://install.duckdb.org | sh
```

### Download parquet files from Substreams
```bash
substreams-sink-files run eth.substreams.pinax.network:443 \
    https://spkg.io/pinax-network/erc20-balances-v1.5.0.spkg map_events \
    "./out" 21525891:21526891 \
    --encoder parquet --file-block-count 1
```

Folder structure:

- `./out`
  - `./balance_changes`
  - `./transfers`

### Query the parquet files with DuckDB

> Select the top sending addresses for `DAI` by total transferred value.

```sql
SELECT
    "from",
    count() as total_transfers,
    sum(value::HUGEINT / 10**18)::DECIMAL(18, 2) as total_value
FROM read_parquet('./out/transfers/*.parquet') AS t
WHERE contract = '6b175474e89094c44da98b954eedeac495271d0f' -- DAI
GROUP BY "from" ORDER BY total_value DESC LIMIT 30;
```
