-- Native balance changes --
CREATE TABLE IF NOT EXISTS native_balance_changes (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- balance change --
   contract             FixedString(42) COMMENT 'Native contract address',
   owner                FixedString(42) COMMENT 'Native owner address',
   old_balance          UInt256 COMMENT 'Native owner old balance',
   new_balance          UInt256 COMMENT 'Native owner new balance',

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_owner              (owner)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_old_balance        (old_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance        (new_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)           TYPE set(20) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, `index`)
ORDER BY (date, block_num, `index`);