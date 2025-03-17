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
   INDEX idx_balance_changes_transaction_id     (transaction_id)  TYPE bloom_filter GRANULARITY 4,
   INDEX idx_balance_changes_contract           (contract)        TYPE bloom_filter GRANULARITY 4,
   INDEX idx_balance_changes_owner              (owner)           TYPE bloom_filter GRANULARITY 4
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
   INDEX idx_transfers_transaction_id     (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_contract     (contract)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_from         (`from`)       TYPE bloom_filter GRANULARITY 4,
   INDEX idx_transfers_to           (`to`)         TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-- ERC-20 contracts metadata events --
CREATE TABLE IF NOT EXISTS contract_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- contract --
   address              FixedString(42),
   name                 String,
   symbol               String,
   decimals             UInt8,

   -- indexes --
   INDEX idx_contract_changes_name      (name)        TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract_changes_symbol    (symbol)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract_changes_decimals  (decimals)    TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address, block_num)
ORDER BY (address, block_num);

-- latest contract creations --
CREATE TABLE IF NOT EXISTS contract_creations  (
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
   address              FixedString(42),

   -- indexes --
   INDEX idx_contract_creations_transaction_id      (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract_creations_from      (`from`)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract_creations_to        (`to`)        TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address)
ORDER BY (address);

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(42),
   owner                FixedString(42),
   new_balance          UInt256,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract)
ORDER BY (owner, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_mv
TO balances AS
SELECT * FROM balance_changes;

-- latest balances by contract/owner --
CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_contract
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (contract, owner)
ORDER BY (contract, owner)
AS
SELECT * FROM balances;

-- latest balances by owner/contract/date --
CREATE TABLE IF NOT EXISTS balances_by_date  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- balance change --
   contract             FixedString(42),
   owner                FixedString(42),
   new_balance          UInt256,

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (owner, contract, date)
ORDER BY (owner, contract, date);

CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_date_mv
TO balances_by_date AS
SELECT * FROM balance_changes;

-- latest ERC-20 contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- contract --
   address              FixedString(42),
   name                 String,
   symbol               String,
   decimals             UInt8,

   -- indexes --
   INDEX idx_contracts_name         (name)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_symbol       (symbol)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contracts_decimals     (decimals)  TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS contracts_mv
TO contracts AS
SELECT * FROM contract_changes;


-- prices pairs created --
CREATE TABLE IF NOT EXISTS pairs_created  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),
   creator              FixedString(42),
   `to`                 FixedString(42),

   -- log --
   factory              FixedString(42),

   -- pair created --
   token0               FixedString(42),
   token1               FixedString(42),
   pair                 FixedString(42),

   -- indexes --
   INDEX idx_pairs_created_transaction_id    (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_pairs_created_token0            (token0)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_pairs_created_token1            (token1)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_pairs_created_creator           (creator)            TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (factory, pair)
ORDER BY (factory, pair);

-- prices sync changes --
CREATE TABLE IF NOT EXISTS sync_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- sync --
   reserve0             FixedString(42),
   reserve1             FixedString(42),

   -- indexes --
   INDEX idx_sync_changes_transaction_id     (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sync_changes_reserve0           (reserve0)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sync_changes_reserve1           (reserve1)           TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-- prices swaps --
CREATE TABLE IF NOT EXISTS swaps  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42),

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- swaps --
   amount0_in           UInt256,
   amount0_out          UInt256,
   amount1_in           UInt256,
   amount1_out          UInt256,
   sender               FixedString(42),
   `to`                 FixedString(42),

   -- indexes --
   INDEX idx_swaps_transaction_id   (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_swaps_address          (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_swaps_sender           (sender)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_swaps_to               (`to`)               TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);


-- latest Uniswap V2 liquidity pool syncs --
CREATE TABLE IF NOT EXISTS syncs  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- log --
   address              FixedString(42),

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- sync --
   reserve0             FixedString(42),
   reserve1             FixedString(42),

   INDEX idx_syncs_reserve0           (reserve0)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_syncs_reserve1           (reserve1)           TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS syncs_mv
TO syncs AS
SELECT * FROM sync_changes