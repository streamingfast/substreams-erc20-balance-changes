# ERC-20: `Substreams`

Substreams for tracking ERC-20 token balances & transfers for EVM blockchains.

> Ethereum, Base, BSC, Polygon, ArbitrumOne, Optimism, etc..

Includes the following:

- [x] Apache Parquet - File Sink (Protobuf Map modules)
- [x] Clickhouse SQL Sink
- [ ] Postgres SQL Sink
- [ ] Graph Node - Subgraphs (SpS)

## Substreams Packages

- [x] ERC-20 Balances & Transfers
- [x] Native Balances
- [ ] Native Transfers
- [ ] ERC-20 Contract Metadata
- [ ] ERC-20 Supply

## Substreams Graph

```mermaid
graph TD;
  graph_out[map: graph_out];
  erc20_balances:map_events --> graph_out;
  erc20_balances:map_events --> db_out;
  erc20_balances:map_events[map: erc20_balances:map_events];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> erc20_balances:map_events;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> erc20_balances:map_events;
```
