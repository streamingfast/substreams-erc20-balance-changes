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

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS native_balances AS erc20_balances
ENGINE = ReplacingMergeTree(global_sequence)
PRIMARY KEY (address)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS native_balances_mv
TO native_balances AS
SELECT * FROM native_balance_changes;
