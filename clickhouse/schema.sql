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

   -- ordering --
   ordinal              UInt64, -- storage_change.ordinal or balance_change.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 contract address',
   owner                FixedString(42) COMMENT 'ERC-20 owner address',
   old_balance          UInt256 COMMENT 'ERC-20 owner old balance',
   new_balance          UInt256 COMMENT 'ERC-20 owner new balance',

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

-------------------------------------------------
-- Transfer events                      --
-------------------------------------------------
CREATE TABLE IF NOT EXISTS transfers  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

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
   INDEX idx_value              (value)              TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)          TYPE set(20) GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, `index`)
ORDER BY (date, block_num, `index`);

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

-- latest contract creations --
CREATE TABLE IF NOT EXISTS contract_creations  (
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
   `from`               FixedString(42) COMMENT 'ERC-20 creator address',
   `to`                 FixedString(42),

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',

   -- indexes --
   INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,
   INDEX idx_date               (date)                TYPE minmax GRANULARITY 4,
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)                TYPE bloom_filter GRANULARITY 4,
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

-- latest ERC-20 contracts --
CREATE TABLE IF NOT EXISTS contracts  (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 String COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               String COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             UInt8 COMMENT 'ERC-20 contract decimals (18 by default)',


   -- indexes --
   INDEX idx_block_num     (block_num)       TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp     (timestamp)       TYPE minmax GRANULARITY 4,
   INDEX idx_date          (date)            TYPE minmax GRANULARITY 4,
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


-- prices pairs created --
CREATE TABLE IF NOT EXISTS uniswap_v2_pairs_created (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV2Pair factory address', -- log.address

   -- pair created --
   token0               FixedString(42) COMMENT 'UniswapV2Pair token0 address',
   token1               FixedString(42) COMMENT 'UniswapV2Pair token1 address',
   pair                 FixedString(42) COMMENT 'UniswapV2Pair pair address',

   -- indexes --
   INDEX idx_block_num        (block_num)          TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp        (timestamp)          TYPE minmax GRANULARITY 4,
   INDEX idx_date             (date)               TYPE minmax GRANULARITY 4,
   INDEX idx_transaction_id   (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token0           (token0)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token1           (token1)             TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address, pair)
ORDER BY (address, pair);

-- Uniswap::V2::Pair:Sync --
CREATE TABLE IF NOT EXISTS uniswap_v2_sync_changes  (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV2Pair pair address', -- log.address

   -- sync --
   reserve0             UInt256 COMMENT 'UniswapV2Pair token0 reserve',
   reserve1             UInt256 COMMENT 'UniswapV2Pair token1 reserve',

   -- indexes --
   INDEX idx_sync_changes_transaction_id     (transaction_id)  TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sync_changes_reserve0_minmax    (reserve0)        TYPE minmax       GRANULARITY 4,
   INDEX idx_sync_changes_reserve1_minmax    (reserve1)        TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-- Uniswap::V2::Pair:Swap --
CREATE TABLE IF NOT EXISTS uniswap_v2_swaps (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV2Pair pair address', -- log.address

   -- swaps --
   amount0_in           UInt256 COMMENT 'UniswapV2Pair token0 amount in',
   amount0_out          UInt256 COMMENT 'UniswapV2Pair token0 amount out',
   amount1_in           UInt256 COMMENT 'UniswapV2Pair token1 amount in',
   amount1_out          UInt256 COMMENT 'UniswapV2Pair token1 amount out',
   sender               FixedString(42) COMMENT 'UniswapV2Pair sender address',
   `to`                 FixedString(42) COMMENT 'UniswapV2Pair recipient address',

   -- indexes --
   INDEX idx_transaction_id   (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address          (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sender           (sender)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to               (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_amount0_in       (amount0_in)         TYPE minmax       GRANULARITY 4,
   INDEX idx_amount0_out      (amount0_out)        TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1_in       (amount1_in)         TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1_out      (amount1_out)        TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-- latest Uniswap::V2::Pair:Sync --
CREATE TABLE IF NOT EXISTS uniswap_v2_syncs  (
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
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS uniswap_v2_syncs_mv
TO uniswap_v2_syncs AS
SELECT * FROM uniswap_v2_sync_changes;

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


-- Uniswap::V3::Pool:Swap --
CREATE TABLE IF NOT EXISTS uniswap_v3_swaps (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV3Pool pool address', -- log.address

   -- swaps --
   sender               FixedString(42) COMMENT 'UniswapV3Pool sender address',
   recipient            FixedString(42) COMMENT 'UniswapV3Pool recipient address',
   amount0              Int256 COMMENT 'UniswapV3Pool token0 amount',
   amount1              Int256 COMMENT 'UniswapV3Pool token1 amount',
   sqrt_price_x96       UInt256 COMMENT 'UniswapV3Pool sqrt price x96',
   tick                 Int32 COMMENT 'UniswapV3Pool tick',
   liquidity            UInt128 COMMENT 'UniswapV3Pool liquidity',

   -- indexes --
   INDEX idx_transaction_id   (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address          (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sender           (sender)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_recipient        (recipient)          TYPE bloom_filter GRANULARITY 4,
   INDEX idx_amount0           (amount0)           TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1           (amount1)           TYPE minmax       GRANULARITY 4,
   INDEX idx_sqrt_price_x96    (sqrt_price_x96)    TYPE minmax       GRANULARITY 4,
   INDEX idx_tick              (tick)              TYPE minmax       GRANULARITY 4,
   INDEX idx_liquidity         (liquidity)         TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (date, block_num, ordinal)
ORDER BY (date, block_num, ordinal);

-- Uniswap::V3::Pool:Initialize --
CREATE TABLE IF NOT EXISTS uniswap_v3_initializes (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV3Pool pool address', -- log.address

   -- initializes --
   sqrt_price_x96       UInt256 COMMENT 'UniswapV3Pool sqrt price x96',
   tick                 Int32 COMMENT 'UniswapV3Pool tick',

   -- indexes --
   INDEX idx_transaction_id    (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sqrt_price_x96    (sqrt_price_x96)    TYPE minmax       GRANULARITY 4,
   INDEX idx_tick              (tick)              TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address)
ORDER BY (address);

-- Uniswap::V3::Factory:PoolCreated --
CREATE TABLE IF NOT EXISTS uniswap_v3_pools_created (
   -- block --
   block_num            UInt32,
   block_hash           FixedString(66),
   timestamp            DateTime(0, 'UTC'),
   date                 Date,

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- log --
   address              FixedString(42) COMMENT 'UniswapV3Pool pool address', -- log.address

   -- initializes --
   token0               FixedString(42) COMMENT 'UniswapV3Pool token0 address',
   token1               FixedString(42) COMMENT 'UniswapV3Pool token1 address',
   pool                 FixedString(42) COMMENT 'UniswapV3Pool pool address',
   tick_spacing         Int32 COMMENT 'UniswapV3Pool tick spacing (e.g., 60)',
   fee                  UInt32 COMMENT 'UniswapV3Pool fee (e.g., 3000 represents 0.30%)',

   -- indexes --
   INDEX idx_transaction_id    (transaction_id)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token0            (token0)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token1            (token1)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_tick_spacing      (tick_spacing)      TYPE minmax       GRANULARITY 4,
   INDEX idx_fee               (fee)               TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address, pool)
ORDER BY (address, pool);
