-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-------------------------------------------------
-- Balance changes events                      --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS balance_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(64),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(64),

   -- call --
   call_index           UInt32,

   -- log --
   log_index            UInt32,
   log_block_index      UInt32,
   log_ordinal          UInt64,

   -- storage change --
   storage_key          FixedString(64),
   storage_ordinal      UInt64,

   -- balance change --
   contract             FixedString(40),
   owner                FixedString(40),
   old_balance          UInt256,
   new_balance          UInt256,

   -- indexing --
   version              UInt64, -- latest version of the balance change (block_num << 32 + storage_ordinal)

   -- debug --
   balance_change_type  Int32
)
ENGINE = ReplacingMergeTree
PARTITION BY date
PRIMARY KEY (block_num, storage_ordinal)
ORDER BY (block_num, storage_ordinal);

-- create a bloom-filter index for these high-cardinality string columns
CREATE INDEX IF NOT EXISTS idx_balance_changes_contract ON balance_changes (contract) TYPE bloom_filter GRANULARITY 4;
CREATE INDEX IF NOT EXISTS idx_balance_changes_owner    ON balance_changes (owner)   TYPE bloom_filter GRANULARITY 4;

-------------------------------------------------
-- Transfer events                      --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS transfers  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(64),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(64),

   -- call --
   call_index           UInt32,
   call_address         FixedString(40),

   -- log --
   log_index            UInt32,
   log_block_index      UInt32,
   log_ordinal          UInt64,

   -- transfer --
   contract             FixedString(40), -- log.address
   `from`               FixedString(40),
   `to`                 FixedString(40),
   value                UInt256,

   -- debug --
   transfer_type        Int32
)
ENGINE = ReplacingMergeTree
PARTITION BY date
PRIMARY KEY (block_num, log_block_index)
ORDER BY (block_num, log_block_index);

-- create a bloom-filter index for these high-cardinality string columns
CREATE INDEX IF NOT EXISTS idx_transfers_contract ON transfers (contract) TYPE bloom_filter GRANULARITY 4;
CREATE INDEX IF NOT EXISTS idx_transfers_from     ON transfers (`from`)   TYPE bloom_filter GRANULARITY 4;
CREATE INDEX IF NOT EXISTS idx_transfers_to       ON transfers (`to`)     TYPE bloom_filter GRANULARITY 4;

-- latest balances by account --
CREATE MATERIALIZED VIEW IF NOT EXISTS balances
ENGINE = ReplacingMergeTree(version)
ORDER BY (owner, contract)
POPULATE
AS
SELECT * FROM balance_changes;

-- latest balances by account & by date --
CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_date
ENGINE = ReplacingMergeTree(version)
ORDER BY (owner, contract, date)
POPULATE
AS
SELECT * FROM balance_changes;
