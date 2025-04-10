-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances (
   -- block --
   block_num            UInt32,
   timestamp            DateTime(0, 'UTC'),

   -- ordering --
   global_sequence      UInt64, -- block_num << 32 + index

   -- balance change --
   contract             FixedString(42) DEFAULT '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee' COMMENT 'contract address',
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
CREATE MATERIALIZED VIEW IF NOT EXISTS balances_by_contract
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (contract, address)
ORDER BY (contract, address)
AS
SELECT * FROM balances;