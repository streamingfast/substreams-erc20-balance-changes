-- Historical ERC-20 balances by address/contract --
CREATE TABLE IF NOT EXISTS historical_erc20_balances (
   -- block --
   timestamp            DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

   -- balance change --
   contract             FixedString(42) COMMENT 'contract address',
   address              FixedString(42) COMMENT 'wallet address',

   -- balance --
   open           AggregateFunction(argMin, Float64, UInt64),
   high           SimpleAggregateFunction(max, Float64),
   low            SimpleAggregateFunction(min, Float64),
   close          AggregateFunction(argMax, Float64, UInt64),
   uaw            AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses that changed balance in the window',
   transactions   AggregateFunction(sum, UInt8) COMMENT 'number of transactions that changed balance in the window'
)
ENGINE = AggregatingMergeTree
PRIMARY KEY (address, contract, timestamp)
ORDER BY (address, contract, timestamp);

-- ERC-20 balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_erc20_balances_mv
TO historical_erc20_balances
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   address,
   contract,
   argMinState(toFloat64(new_balance / pow(10, contracts.decimals)), global_sequence) AS open,
   max(toFloat64(new_balance / pow(10, contracts.decimals))) AS high,
   min(toFloat64(new_balance / pow(10, contracts.decimals))) AS low,
   argMaxState(toFloat64(new_balance / pow(10, contracts.decimals)), global_sequence) AS close,
   uniqState(address) AS uaw,
   sumState(1) AS transactions
FROM erc20_balance_changes
JOIN contracts
   ON erc20_balance_changes.contract = contracts.address
GROUP BY address, contract, timestamp;

-- Historical balances by contract/address --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_erc20_balances_by_contract
ENGINE = AggregatingMergeTree
PRIMARY KEY (contract, address, timestamp)
ORDER BY (contract, address, timestamp)
AS
SELECT * FROM historical_erc20_balances;
