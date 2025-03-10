# EVM-Tokens: `Substreams`

Substreams for tracking ERC-20 & Native token balances & transfers for EVM blockchains.

> Ethereum, Base, BSC, Polygon, ArbitrumOne, Optimism, etc..

Includes the following:

- [x] Apache Parquet - File Sink (Protobuf Map modules)
- [x] Clickhouse SQL Sink
- [ ] Postgres SQL Sink
- [ ] Graph Node - Subgraphs (SpS)

## Substreams Packages

- [x] ERC-20 Balances & Transfers
- [x] Native Balances
- [x] Native Transfers
- [ ] ERC-20 Contract Metadata
- [ ] ERC-20 Supply

## Substreams Graph

```mermaid
graph TD;
  graph_out[map: graph_out];
  native:map_events --> map_events;
  erc20:map_events --> map_events;

  map_events --> graph_out;
  map_events --> db_out;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> erc20:map_events;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> native:map_events;
```
