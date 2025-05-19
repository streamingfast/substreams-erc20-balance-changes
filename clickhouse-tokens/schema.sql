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

-- ERC-20 balance changes --
CREATE TABLE IF NOT EXISTS erc20_balance_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   tx_hash       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'ERC-20 caller address', -- call.caller

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 contract address',
   address              FixedString(42) COMMENT 'ERC-20 wallet address',
   old_balance          UInt256 COMMENT 'ERC-20 old balance',
   new_balance          UInt256 COMMENT 'ERC-20 new balance',

   -- debug --
   algorithm            LowCardinality(String),
   trx_type             LowCardinality(String),
   call_type            LowCardinality(String),
   reason               LowCardinality(String) COMMENT 'only available in native_balance_changes',

   -- indexes --
   INDEX idx_tx_hash            (tx_hash)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_old_balance        (old_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance        (new_balance)         TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)           TYPE set(32) GRANULARITY 4,
   INDEX idx_trx_type           (trx_type)            TYPE set(32) GRANULARITY 4,
   INDEX idx_call_type          (call_type)           TYPE set(32) GRANULARITY 4,
   INDEX idx_reason             (reason)              TYPE set(32) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- ERC-20 transfers --
CREATE TABLE IF NOT EXISTS erc20_transfers  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   tx_hash              FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'ERC-20 contract caller address', -- call.caller

   -- transfer --
   contract             FixedString(42) COMMENT 'ERC-20 contract address', -- log.address
   `from`               FixedString(42) COMMENT 'ERC-20 transfer sender address', -- log.topics[1]
   `to`                 FixedString(42) COMMENT 'ERC-20 transfer recipient address', -- log.topics[2]
   value                UInt256 COMMENT 'ERC-20 transfer value', -- log.data

   -- debug --
   algorithm            LowCardinality(String),
   trx_type             LowCardinality(String),
   call_type            LowCardinality(String),

   -- indexes --
   INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_value              (value)              TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)          TYPE set(32) GRANULARITY 4,
   INDEX idx_trx_type           (trx_type)           TYPE set(32) GRANULARITY 4,
   INDEX idx_call_type          (call_type)          TYPE set(32) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);


-- ERC-20 contracts metadata events --
CREATE TABLE IF NOT EXISTS contract_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   tx_hash       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'contract creator/modifier address',

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 String COMMENT '(Optional) ERC-20 contract name (typically 3-8 characters)',
   symbol               String COMMENT '(Optional) ERC-20 contract symbol (typically 3-4 characters)',
   decimals             String COMMENT '(Optional UInt8) ERC-20 contract decimals (18 by default)',

   -- indexes --
   INDEX idx_tx_hash             (tx_hash)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller              (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address             (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_name                (name)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_symbol              (symbol)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_decimals            (decimals)           TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);


-- Native balance changes --
CREATE TABLE IF NOT EXISTS native_balance_changes AS erc20_balance_changes
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- Native transfers --
CREATE TABLE IF NOT EXISTS native_transfers AS erc20_transfers
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);


-- contract creations events --
CREATE TABLE IF NOT EXISTS contract_creations  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   tx_hash       FixedString(66),
   `from`               FixedString(42),
   `to`                 FixedString(42),

   -- call --
   caller               FixedString(42) COMMENT 'contract creator',

   -- contract --
   address              FixedString(42) COMMENT 'contract address',
   hash                 FixedString(66) COMMENT 'unique contract hash',

   -- indexes --
   INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,
   INDEX idx_tx_hash            (tx_hash)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)                TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_hash               (hash)                TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address)
ORDER BY (address);


-- latest Token contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   -- block --
   block_num            SimpleAggregateFunction(max, UInt32) COMMENT 'block number',
   timestamp            SimpleAggregateFunction(max, DateTime(0, 'UTC')),

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 SimpleAggregateFunction(anyLast, Nullable(String)) COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               SimpleAggregateFunction(anyLast, Nullable(String)) COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             SimpleAggregateFunction(anyLast, Nullable(UInt8)) COMMENT 'ERC-20 contract decimals (18 by default)'
)
ENGINE = AggregatingMergeTree
ORDER BY address;

CREATE MATERIALIZED VIEW IF NOT EXISTS contracts_mv
TO contracts AS
SELECT
   block_num,
   timestamp,
   global_sequence,
   address,
   -- replace empty strings with NULLs --
   IF (name = '', Null, name) AS name,
   IF (symbol = '', Null, symbol) AS symbol,
   IF (decimals = '', Null, CAST(decimals AS UInt8)) AS decimals
FROM contract_changes;

-- one time INSERT to populate Native contract --
INSERT INTO contracts (
   block_num,
   timestamp,
   global_sequence,
   address,
   name,
   symbol,
   decimals
)
VALUES (
   0,
   toDateTime(0, 'UTC'),
   0,
   '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee',
   'Native',
   'Native',
   18
);

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances AS erc20_balance_changes
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address, contract)
ORDER BY (address, contract);

-- insert ERC20 balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS erc20_balances_mv
TO balances AS
SELECT * FROM erc20_balance_changes
WHERE algorithm != 'ALGORITHM_BALANCE_NOT_MATCH_TRANSFER'; -- not implemented yet

-- insert Native balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS native_balances_mv
TO balances AS
SELECT * FROM native_balance_changes;

-- latest balances by contract/address --
CREATE TABLE IF NOT EXISTS balances_by_contract AS balances
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (contract, address)
ORDER BY (contract, address);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_contract_mv
TO balances_by_contract AS
SELECT * FROM balances;


-- Historical ERC-20 balances by address/contract --
CREATE TABLE IF NOT EXISTS historical_balances (
   -- block --
   block_num            SimpleAggregateFunction(min, UInt32),
   timestamp            DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

   -- balance change --
   contract             FixedString(42) COMMENT 'contract address',
   address              FixedString(42) COMMENT 'wallet address',

   -- balance --
   open                 AggregateFunction(argMin, UInt256, UInt64),
   high                 SimpleAggregateFunction(max, UInt256),
   low                  SimpleAggregateFunction(min, UInt256),
   close                AggregateFunction(argMax, UInt256, UInt64),
   uaw                  AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses that changed balance in the window',
   transactions         AggregateFunction(sum, UInt8) COMMENT 'number of transactions that changed balance in the window'
)
ENGINE = AggregatingMergeTree
PRIMARY KEY (address, contract, timestamp)
ORDER BY (address, contract, timestamp);

-- ERC-20 balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_erc20_balances_mv
TO historical_balances
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   min(block_num) AS block_num,
   address,
   contract,
   argMinState(new_balance, global_sequence) AS open, -- normalized to wei (18 decimals)
   max(new_balance) AS high, -- normalized to wei (18 decimals)
   min(new_balance) AS low, -- normalized to wei (18 decimals)
   argMaxState(new_balance, global_sequence) AS close, -- normalized to wei (18 decimals)
   uniqState(address) AS uaw,
   sumState(1) AS transactions
FROM erc20_balance_changes
GROUP BY address, contract, timestamp;

-- Native balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_native_balances_mv
TO historical_balances
AS
SELECT
   -- block --
   min(block_num) AS block_num,
   toStartOfHour(timestamp) AS timestamp,

   -- balance change --
   '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee' AS contract,
   address,

   -- balance --
   argMinState(new_balance, global_sequence) AS open,
   max(new_balance) AS high,
   min(new_balance) AS low,
   argMaxState(new_balance, global_sequence) AS close,
   uniqState(address) AS uaw,
   sumState(1) AS transactions
FROM native_balance_changes
GROUP BY address, timestamp;

-- Historical balances by contract/address --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_balances_by_contract
ENGINE = AggregatingMergeTree
PRIMARY KEY (contract, address, timestamp)
ORDER BY (contract, address, timestamp)
AS
SELECT * FROM historical_balances;


-- latest transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- block --
    timestamp			DateTime(0, 'UTC'),
    block_num			UInt32,

    -- ordering --
    `index`             UInt64,
    global_sequence     UInt64,

    -- transaction --
    transaction_id		FixedString(66),

    -- log --
    contract			FixedString(42),

    -- event --
    `from`				FixedString(42),
    `to`				FixedString(42),
    decimals            String,
    symbol              String,
    amount              String,
    value               UInt256,

    -- indexes --
    INDEX idx_transaction_id     (transaction_id)     TYPE bloom_filter GRANULARITY 4,
    INDEX idx_contract           (contract)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_value              (value)              TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (timestamp, block_num, `from`, `to`, `index`);

-- insert ERC20 transfers --
CREATE MATERIALIZED VIEW IF NOT EXISTS erc20_transfers_mv
TO transfers AS
SELECT
    -- block --
    timestamp,
    block_num,

    -- ordering --
    `index`,
    global_sequence,

    -- transaction --
    transaction_id,

    -- log --
    contract,

    -- event --
    `from`,
    `to`,
    decimals,
    symbol,
    toString(t.value) as amount,
    value / pow(10, decimals) AS value
FROM erc20_transfers AS t
LEFT JOIN contracts AS c ON c.address = t.contract;

-- insert Native transfers --
CREATE MATERIALIZED VIEW IF NOT EXISTS native_transfers_mv
TO transfers AS
SELECT
    -- block --
    timestamp,
    block_num,

    -- ordering --
    `index`,
    global_sequence,

    -- transaction --
    transaction_id,

    -- log --
    contract,

    -- event --
    `from`,
    `to`,
    decimals,
    symbol,
    toString(t.value) as amount,
    value / pow(10, decimals) AS value
FROM native_transfers AS t
LEFT JOIN contracts AS c ON c.address = t.contract;

