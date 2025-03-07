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

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   global_sequence      UInt64, -- latest version of the balance change (block_num << 32 + index)

   -- balance change --
   contract             FixedString(40),
   owner                FixedString(40),
   old_balance          UInt256,
   new_balance          UInt256,

   -- metadata --
   type                 LowCardinality(String),

   -- indexes --
   INDEX idx_balance_changes_date     (date)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_balance_changes_contract (contract)  TYPE bloom_filter GRANULARITY 4,
   INDEX idx_balance_changes_owner    (owner)     TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
PARTITION BY date
PRIMARY KEY (block_num, ordinal)
ORDER BY (block_num, ordinal);

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

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   global_sequence      UInt64, -- latest global sequence of the transfer (block_num << 32 + index)

   -- transfer --
   contract             FixedString(40), -- log.address
   `from`               FixedString(40),
   `to`                 FixedString(40),
   value                UInt256,

   -- metadata --
   type                 LowCardinality(String),

   -- indexes --
   INDEX idx_transfers_date     (date)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_contract (contract)  TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_from     (`from`)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_to       (`to`)      TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
PARTITION BY date
PRIMARY KEY (block_num, ordinal)
ORDER BY (block_num, ordinal);

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(40),
   owner                FixedString(40),
   new_balance          UInt256,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- indexes --
   INDEX idx_balances_contract (contract)  TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract)
ORDER BY (owner, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_mv
TO balances AS
SELECT * FROM balance_changes;

-- latest balances by owner/contract/date --
CREATE TABLE IF NOT EXISTS balances_by_date  (
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(40),
   owner                FixedString(40),
   new_balance              UInt256,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- indexes --
   INDEX idx_balances_contract (contract)  TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract, date)
ORDER BY (owner, contract, date);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_date_mv
TO balances_by_date AS
SELECT * FROM balance_changes;

-- remove zero balances --
ALTER TABLE balances MODIFY TTL timestamp WHERE new_balance <= 0;
ALTER TABLE balances_by_date MODIFY TTL timestamp WHERE new_balance <= 0;
