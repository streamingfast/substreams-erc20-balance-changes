-- This file is generated. Do not edit.

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

--ERC-20 balance changes --
CREATE TABLE IF NOT EXISTS balance_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'ERC-20 contract caller address', -- call.caller

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 contract address',
   address              FixedString(42) COMMENT 'ERC-20 wallet address',
   old_balance          UInt256 COMMENT 'ERC-20 wallet address old balance',
   new_balance          UInt256 COMMENT 'ERC-20 wallet address new balance',

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_old_balance        (old_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance        (new_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)           TYPE set(20) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- ERC-20 transfers --
CREATE TABLE IF NOT EXISTS transfers  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'ERC-20 contract caller address', -- call.caller

   -- transfer --
   contract             FixedString(42) COMMENT 'ERC-20 contract address', -- log.address
   `from`               FixedString(42) COMMENT 'ERC-20 transfer sender address', -- log.topics[1]
   `to`                 FixedString(42) COMMENT 'ERC-20 transfer recipient address', -- log.topics[2]
   value                UInt256 COMMENT 'ERC-20 transfer value', -- log.data

   -- debug --
   algorithm            LowCardinality(String),
   algorithm_code       UInt8,

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_value              (value)              TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)          TYPE set(32) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 contract address',
   address              FixedString(42) COMMENT 'ERC-20 wallet address',
   new_balance          UInt256 COMMENT 'ERC-20 new balance',

   -- indexes --
   INDEX idx_block_num     (block_num)       TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp     (timestamp)       TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance   (new_balance)     TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address, contract)
ORDER BY (address, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_mv
TO balances AS
SELECT * FROM balance_changes;

-- latest balances by contract/address --
CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_contract
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (contract, address)
ORDER BY (contract, address)
AS
SELECT * FROM balances;

-- ERC-20 contracts metadata events --
CREATE TABLE IF NOT EXISTS contract_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   -- ordinal              UInt64 COMMENT 'NOT IMPLEMENTED', -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- -- transaction --
   -- transaction_id       FixedString(66) COMMENT 'NOT IMPLEMENTED',
   -- `from`               FixedString(42) COMMENT 'NOT IMPLEMENTED: ERC-20 creator/modifier address',
   -- `to`                 FixedString(42) COMMENT 'NOT IMPLEMENTED',

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 String COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               String COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             UInt8 COMMENT 'ERC-20 contract decimals (18 by default)',

   -- -- debug --
   -- algorithm            LowCardinality(String) COMMENT 'NOT IMPLEMENTED',
   -- algorithm_code       UInt8 COMMENT 'NOT IMPLEMENTED',

   -- indexes --
   -- INDEX idx_transaction_id      (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   -- INDEX idx_from                (`from`)             TYPE bloom_filter GRANULARITY 4,
   -- INDEX idx_to                  (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address             (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_name                (name)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_symbol              (symbol)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_decimals            (decimals)           TYPE minmax GRANULARITY 4,
   -- INDEX idx_algorithm           (algorithm)          TYPE set(2) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, `index`)
ORDER BY (date, block_num, `index`);

-- latest ERC-20 contracts --
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
SELECT * FROM contract_changes;





