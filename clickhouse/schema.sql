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
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   global_sequence      UInt64, -- latest version of the balance change (block_num << 32 + index)

   -- balance change --
   contract             FixedString(42),
   owner                FixedString(42),
   old_balance          UInt256,
   new_balance          UInt256,

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_balance_changes_contract (contract)  TYPE bloom_filter GRANULARITY 4,
   INDEX idx_balance_changes_owner    (owner)     TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-------------------------------------------------
-- Transfer events                      --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS transfers  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   global_sequence      UInt64, -- latest global sequence of the transfer (block_num << 32 + index)

   -- transfer --
   contract             FixedString(42), -- log.address
   `from`               FixedString(42),
   `to`                 FixedString(42),
   value                UInt256,

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_transfers_contract     (contract)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_from         (`from`)       TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_to           (`to`)         TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-------------------------------------------------
-- Contracts creation or updates               --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS contract_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),
   `from`               FixedString(42),
   `to`                 FixedString(42),

   -- contract --
   address              FixedString(42), -- code_changes.address
   name                 String,
   symbol               String,
   decimals             UInt8,

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_contracts_address   (address)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_from      (`from`)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_to        (`to`)        TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_name      (name)        TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_symbol    (symbol)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_decimals  (decimals)    TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, address)
ORDER BY (date, block_num, address);

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(42),
   owner                FixedString(42),
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
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(42),
   owner                FixedString(42),
   new_balance          UInt256,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- indexes --
   INDEX idx_balances_by_date_contract (contract)  TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract, date)
ORDER BY (owner, contract, date);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_date_mv
TO balances_by_date AS
SELECT * FROM balance_changes;


-- latest contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   `from`               FixedString(42),
   `to`                 FixedString(42),

   -- contract --
   address              FixedString(42), -- code_change.address
   name                 String,
   symbol               String,
   decimals             UInt8,

   -- indexes --
   INDEX idx_contracts_name         (name)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_from         (`from`)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_to           (`to`)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_symbol       (symbol)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_decimals     (decimals)  TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS contracts_mv
TO contracts AS
SELECT * FROM contract_changes;