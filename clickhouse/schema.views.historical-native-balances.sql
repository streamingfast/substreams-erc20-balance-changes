-- Historical ERC-20 balances by address/contract --
CREATE TABLE IF NOT EXISTS historical_native_balances as historical_erc20_balances
ENGINE = AggregatingMergeTree
PRIMARY KEY (address, timestamp)
ORDER BY (address, timestamp);

-- Native balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_native_balances_mv
TO historical_native_balances
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   address,
   argMinState(toFloat64(new_balance / pow(10, 18)), global_sequence) AS open,
   max(toFloat64(new_balance / pow(10, 18))) AS high,
   min(toFloat64(new_balance / pow(10, 18))) AS low,
   argMaxState(toFloat64(new_balance / pow(10, 18)), global_sequence) AS close,
   uniqState(address) AS uaw,
   sumState(1) AS transactions
FROM native_balance_changes
GROUP BY address, timestamp;
