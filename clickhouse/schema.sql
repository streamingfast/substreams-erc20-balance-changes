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
   transaction_id       FixedString(66),

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
   reason               LowCardinality(String),

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
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
   trx_type             LowCardinality(String),
   call_type            LowCardinality(String),

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_contract           (contract)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_value              (value)              TYPE minmax GRANULARITY 4,
   INDEX idx_algorithm          (algorithm)          TYPE set(32) GRANULARITY 4,
   INDEX idx_trx_type           (trx_type)            TYPE set(32) GRANULARITY 4,
   INDEX idx_call_type          (call_type)           TYPE set(32) GRANULARITY 4,
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
   transaction_id       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'contract creator/modifier address',

   -- contract --
   address              FixedString(42) COMMENT 'ERC-20 contract address',
   name                 String COMMENT 'ERC-20 contract name (typically 3-8 characters)',
   symbol               String COMMENT 'ERC-20 contract symbol (typically 3-4 characters)',
   decimals             UInt8 COMMENT 'ERC-20 contract decimals (18 by default)',

   -- indexes --
   INDEX idx_transaction_id      (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller              (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address             (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_name                (name)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_symbol              (symbol)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_decimals            (decimals)           TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

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
   transaction_id       FixedString(66),
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
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_from               (`from`)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to                 (`to`)                TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_hash               (hash)                TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address)
ORDER BY (address);






-- prices pairs created --
CREATE TABLE IF NOT EXISTS uniswap_v2_pairs_created (
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
   caller               FixedString(42) COMMENT 'factory creator', -- call.caller

   -- log --
   address              FixedString(42) COMMENT 'UniswapV2Pair factory address', -- log.address

   -- pair created --
   token0               FixedString(42) COMMENT 'UniswapV2Pair token0 address',
   token1               FixedString(42) COMMENT 'UniswapV2Pair token1 address',
   pair                 FixedString(42) COMMENT 'UniswapV2Pair pair address',

   -- indexes --
   INDEX idx_block_num        (block_num)          TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp        (timestamp)          TYPE minmax GRANULARITY 4,
   INDEX idx_transaction_id   (transaction_id)     TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token0           (token0)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token1           (token1)             TYPE bloom_filter GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address, pair)
ORDER BY (address, pair);

-- Uniswap::V2::Pair:Sync --
CREATE TABLE IF NOT EXISTS uniswap_v2_syncs  (
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
   caller               FixedString(42) COMMENT 'caller address', -- call.caller

   -- log --
   address              FixedString(42) COMMENT 'UniswapV2Pair pair address', -- log.address

   -- sync --
   reserve0             UInt256 COMMENT 'UniswapV2Pair token0 reserve',
   reserve1             UInt256 COMMENT 'UniswapV2Pair token1 reserve',

   -- indexes --
   INDEX idx_transaction_id     (transaction_id)      TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller             (caller)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_reserve0_minmax    (reserve0)            TYPE minmax       GRANULARITY 4,
   INDEX idx_reserve1_minmax    (reserve1)            TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- Uniswap::V2::Pair:Swap --
CREATE TABLE IF NOT EXISTS uniswap_v2_swaps (
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
   caller               FixedString(42) COMMENT 'caller address', -- call.caller

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
   INDEX idx_caller           (caller)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address          (address)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sender           (sender)             TYPE bloom_filter GRANULARITY 4,
   INDEX idx_to               (`to`)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_amount0_in       (amount0_in)         TYPE minmax       GRANULARITY 4,
   INDEX idx_amount0_out      (amount0_out)        TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1_in       (amount1_in)         TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1_out      (amount1_out)        TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);



-- Uniswap::V3::Pool:Swap --
CREATE TABLE IF NOT EXISTS uniswap_v3_swaps (
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
   caller               FixedString(42) COMMENT 'caller address', -- call.caller

   -- swaps --
   address              FixedString(42) COMMENT 'UniswapV3Pool pool address', -- log.address
   sender               FixedString(42) COMMENT 'UniswapV3Pool sender address',
   recipient            FixedString(42) COMMENT 'UniswapV3Pool recipient address',
   amount0              Int256 COMMENT 'UniswapV3Pool token0 amount',
   amount1              Int256 COMMENT 'UniswapV3Pool token1 amount',
   sqrt_price_x96       UInt256 COMMENT 'UniswapV3Pool sqrt price x96',
   tick                 Int32 COMMENT 'UniswapV3Pool tick',
   liquidity            UInt128 COMMENT 'UniswapV3Pool liquidity',

   -- indexes --
   INDEX idx_transaction_id    (transaction_id)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller            (caller)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_address           (address)           TYPE bloom_filter GRANULARITY 4,
   INDEX idx_sender            (sender)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_recipient         (recipient)         TYPE bloom_filter GRANULARITY 4,
   INDEX idx_amount0           (amount0)           TYPE minmax       GRANULARITY 4,
   INDEX idx_amount1           (amount1)           TYPE minmax       GRANULARITY 4,
   INDEX idx_sqrt_price_x96    (sqrt_price_x96)    TYPE minmax       GRANULARITY 4,
   INDEX idx_tick              (tick)              TYPE minmax       GRANULARITY 4,
   INDEX idx_liquidity         (liquidity)         TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

-- Uniswap::V3::Pool:Initialize --
CREATE TABLE IF NOT EXISTS uniswap_v3_initializes (
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
   caller               FixedString(42) COMMENT 'caller address', -- call.caller

   -- initializes --
   address              FixedString(42) COMMENT 'UniswapV3Pool pool address', -- log.address
   sqrt_price_x96       UInt256 COMMENT 'UniswapV3Pool sqrt price x96',
   tick                 Int32 COMMENT 'UniswapV3Pool tick',

   -- indexes --
   INDEX idx_transaction_id    (transaction_id)    TYPE bloom_filter    GRANULARITY 4,
   INDEX idx_caller            (caller)            TYPE bloom_filter    GRANULARITY 4,
   INDEX idx_sqrt_price_x96    (sqrt_price_x96)    TYPE minmax          GRANULARITY 4,
   INDEX idx_tick              (tick)              TYPE minmax          GRANULARITY 4,
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

   -- ordering --
   ordinal              UInt64, -- log.ordinal
   `index`              UInt64, -- relative index
   global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

   -- transaction --
   transaction_id       FixedString(66),

   -- call --
   caller               FixedString(42) COMMENT 'caller address', -- call.caller

   -- initializes --
   address              FixedString(42) COMMENT 'UniswapV3Pool factory address', -- log.address
   token0               FixedString(42) COMMENT 'UniswapV3Pool token0 address',
   token1               FixedString(42) COMMENT 'UniswapV3Pool token1 address',
   pool                 FixedString(42) COMMENT 'UniswapV3Pool pool address',
   tick_spacing         Int32 COMMENT 'UniswapV3Pool tick spacing (e.g., 60)',
   fee                  UInt32 COMMENT 'UniswapV3Pool fee (e.g., 3000 represents 0.30%)',

   -- indexes --
   INDEX idx_transaction_id    (transaction_id)    TYPE bloom_filter GRANULARITY 4,
   INDEX idx_caller            (caller)            TYPE bloom_filter    GRANULARITY 4,
   INDEX idx_token0            (token0)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_token1            (token1)            TYPE bloom_filter GRANULARITY 4,
   INDEX idx_tick_spacing      (tick_spacing)      TYPE minmax       GRANULARITY 4,
   INDEX idx_fee               (fee)               TYPE minmax       GRANULARITY 4,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (address, pool)
ORDER BY (address, pool);


-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 & Native contract address',
   address              FixedString(42) COMMENT 'wallet address',
   new_balance          UInt256 COMMENT 'new balance',

   -- indexes --
   INDEX idx_block_num     (block_num)       TYPE minmax GRANULARITY 4,
   INDEX idx_timestamp     (timestamp)       TYPE minmax GRANULARITY 4,
   INDEX idx_new_balance   (new_balance)     TYPE minmax GRANULARITY 4,
)
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address, contract)
ORDER BY (address, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS erc20_balances_mv
TO balances AS
SELECT * FROM erc20_balance_changes;

CREATE MATERIALIZED VIEW IF NOT EXISTS native_balances_mv
TO balances AS
SELECT * FROM native_balance_changes;

-- latest balances by contract/address --
CREATE MATERIALIZED VIEW IF NOT EXISTS erc20_balances_by_contract
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (contract, address)
ORDER BY (contract, address)
AS
SELECT * FROM balances;

