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

   -- transfer --
   `from`               FixedString(40),
   `to`                 FixedString(40),
   value                UInt256,

   -- indexing --
   version              UInt64,

   -- debug --
   balance_change_type  Int32
)
ENGINE = ReplacingMergeTree PRIMARY KEY (block_num, storage_ordinal)
ORDER BY (block_num, storage_ordinal);

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
ENGINE = ReplacingMergeTree PRIMARY KEY (block_num, log_block_index)
ORDER BY (block_num, log_block_index);
