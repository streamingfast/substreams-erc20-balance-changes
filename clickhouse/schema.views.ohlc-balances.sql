-- OHLC balances by owner/contract --
CREATE TABLE IF NOT EXISTS ohlc_balances (
   -- block --
   timestamp             DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

   -- balance change --
   contract             FixedString(42) COMMENT 'ERC-20 & Native contract address',
   address              FixedString(42) COMMENT 'wallet address',

   -- balance --
   open   AggregateFunction(argMin, Decimal(76, 0), UInt64),
   high   SimpleAggregateFunction(max, Decimal(76, 0)),
   low    SimpleAggregateFunction(min, Decimal(76, 0)),
   close  AggregateFunction(argMax, Decimal(76, 0), UInt64)
)
ENGINE = AggregatingMergeTree
PRIMARY KEY (address, contract, timestamp)
ORDER BY (address, contract, timestamp);

-- ERC-20 balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_erc20_balances_mv
TO ohlc_balances
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   address,
   contract,
   argMinState(toDecimal256(new_balance, 0), global_sequence) AS open,
   max(toDecimal256(new_balance, 0)) AS high,
   min(toDecimal256(new_balance, 0)) AS low,
   argMaxState(toDecimal256(new_balance, 0), global_sequence) AS close
FROM erc20_balance_changes
GROUP BY address, contract, timestamp;

-- Native balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_native_balances_mv
TO ohlc_balances
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   address,
   contract,
   argMinState(toDecimal256(new_balance, 0), global_sequence) AS open,
   max(toDecimal256(new_balance, 0)) AS high,
   min(toDecimal256(new_balance, 0)) AS low,
   argMaxState(toDecimal256(new_balance, 0), global_sequence) AS close
FROM native_balance_changes
GROUP BY address, contract, timestamp;

-- OHLC balances by contract/owner --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_balances_by_contract
ENGINE = AggregatingMergeTree
PRIMARY KEY (contract, address, timestamp)
ORDER BY (contract, address, timestamp)
AS
SELECT * FROM ohlc_balances;