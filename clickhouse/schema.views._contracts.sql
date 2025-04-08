-- latest Token contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   -- block --
   block_num            SimpleAggregateFunction(max, UInt32) COMMENT 'block number',
   timestamp            SimpleAggregateFunction(max, DateTime(0, 'UTC')),

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 SimpleAggregateFunction(anyLast, String) COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               SimpleAggregateFunction(anyLast, String) COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             SimpleAggregateFunction(anyLast, UInt8) COMMENT 'ERC-20 contract decimals (18 by default)'
)
ENGINE = AggregatingMergeTree
ORDER BY address;

CREATE MATERIALIZED VIEW IF NOT EXISTS contracts_mv
TO contracts AS
SELECT * FROM contract_changes;