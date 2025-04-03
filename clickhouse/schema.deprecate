-- latest balances by owner/contract/date --
CREATE TABLE IF NOT EXISTS balances_by_date  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 contract address',
   owner                FixedString(42) COMMENT 'ERC-20 owner address',
   new_balance          UInt256 COMMENT 'ERC-20 owner new balance',

   -- indexes --
   INDEX idx_block_num     (block_num)       TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp     (timestamp)       TYPE minmax GRANULARITY 4,
   INDEX idx_date          (date)            TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance   (new_balance)     TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract, date)
ORDER BY (owner, contract, date);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_date_mv
TO balances_by_date AS
SELECT * FROM balance_changes;


-- latest by date Uniswap::V2::Pair:Sync --
CREATE TABLE IF NOT EXISTS uniswap_v2_syncs_by_date  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- log --
   address              FixedString(42),

   -- sync --
   reserve0             UInt256,
   reserve1             UInt256,

   -- indexes --
   INDEX idx_reserve0       (reserve0)         TYPE minmax       GRANULARITY 4,
   INDEX idx_reserve1       (reserve1)         TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address, date)
ORDER BY (address, date);

CREATE MATERIALIZED VIEW IF NOT EXISTS uniswap_v2_syncs_by_date_mv
TO uniswap_v2_syncs_by_date AS
SELECT * FROM uniswap_v2_syncs;