-- OHLC prices including Uniswap V2 & V3 --
CREATE TABLE IF NOT EXISTS ohlc_prices (
   -- block --
   timestamp            DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

   -- pool --
   factory              FixedString(42) COMMENT 'factory address',
   pool                 FixedString(42) COMMENT 'pool address',
   token0               FixedString(42) COMMENT 'token0 address',
   token1               FixedString(42) COMMENT 'token1 address',

   -- swaps --
   open                 AggregateFunction(argMin, Float64, UInt64),
   high                 SimpleAggregateFunction(max, Float64),
   low                  SimpleAggregateFunction(min, Float64),
   close                AggregateFunction(argMax, Float64, UInt64),
   uaw                  AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses in the window',
   transactions         AggregateFunction(sum, UInt8) COMMENT 'number of transactions in the window',
   volume               AggregateFunction(sum, Float64) COMMENT 'total volume in token0 currency',
)
ENGINE = AggregatingMergeTree
PRIMARY KEY (factory, pool, token0, token1, timestamp)
ORDER BY (factory, pool, token0, token1, timestamp);

-- Swaps --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_swaps_mv
TO ohlc_prices
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   factory,
   s.pool AS pool,
   p.token0 as token0,
   p.token1 as token1,
   argMinState(price, s.global_sequence) AS open,
   quantileDeterministic(0.95)(price, s.global_sequence) AS high,
   quantileDeterministic(0.05)(price, s.global_sequence) AS low,
   argMaxState(price, s.global_sequence) AS close,
   uniqState(sender) AS uaw,
   sumState(1) AS transactions,
   sumState(toDecimal256(amount0, 18) / pow(10, c.decimals) ) AS volume
FROM swaps AS s
JOIN pools AS p
   ON s.pool = p.pool
JOIN contracts AS c0
   ON p.token0 = c.address
GROUP BY factory, pool, token0, token1, timestamp;

-- Swaps (Inverse) --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_swaps_inverse_mv
TO ohlc_prices
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   factory,
   s.pool AS pool,
   p.token1 as token0,
   p.token0 as token1,
   argMinState(1 / price, s.global_sequence) AS open,
   quantileDeterministic(0.95)(1 / price, s.global_sequence) AS high,
   quantileDeterministic(0.05)(1 / price, s.global_sequence) AS low,
   argMaxState(1 / price, s.global_sequence) AS close,
   uniqState(sender) AS uaw,
   sumState(0) AS transactions,
   sumState(toDecimal256(amount1, 18) / pow(10, c.decimals) ) AS volume
FROM swaps AS s
JOIN pools AS p
   ON s.pool = p.pool
JOIN contracts AS c1
   ON p.token1 = c.address
GROUP BY factory, pool, token0, token1, timestamp;
