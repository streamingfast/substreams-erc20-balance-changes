-- latest Token contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 String COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               String COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             UInt8 COMMENT 'ERC-20 contract decimals (18 by default)',

   -- indexes --
   INDEX idx_block_num     (block_num)        TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp     (timestamp)        TYPE minmax GRANULARITY 4,
   INDEX idx_name          (name)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_symbol        (symbol)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_decimals      (decimals)         TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS contracts_mv
TO contracts AS
SELECT * FROM erc20_contract_changes;