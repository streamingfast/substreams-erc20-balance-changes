-- OHLC prices including Uniswap V2 & V3 --
CREATE TABLE IF NOT EXISTS ohlc_prices (
   -- block --
   block_num            SimpleAggregateFunction(min, UInt32),
   timestamp            DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

   -- pool --
   pool                 FixedString(42) COMMENT 'pool address',

   -- swaps --
   open0                AggregateFunction(argMin, Float64, UInt64),
   high0                SimpleAggregateFunction(max, Float64),
   low0                 SimpleAggregateFunction(min, Float64),
   close0               AggregateFunction(argMax, Float64, UInt64),
   volume0              AggregateFunction(sum, Float64),

   -- swaps (inverse) --
   open1                AggregateFunction(argMin, Float64, UInt64),
   high1                SimpleAggregateFunction(max, Float64),
   low1                 SimpleAggregateFunction(min, Float64),
   close1               AggregateFunction(argMax, Float64, UInt64),
   volume1              AggregateFunction(sum, Float64),

   -- universal --
   uaw                  AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses in the window',
   transactions         AggregateFunction(sum, UInt8) COMMENT 'number of transactions in the window'
)
ENGINE = AggregatingMergeTree
PRIMARY KEY (pool, timestamp)
ORDER BY (pool, timestamp);

-- Swaps --
CREATE MATERIALIZED VIEW IF NOT EXISTS ohlc_swaps_mv
TO ohlc_prices
AS
SELECT
   toStartOfHour(timestamp) AS timestamp,
   min(block_num) AS block_num,
   pool,

   -- swaps --
   argMinState(price, global_sequence) AS open0,
   quantileExact(0.95)(price) AS high0,
   quantileExact(0.05)(price) AS low0,
   argMaxState(price, global_sequence) AS close0,
   sumState(toDecimal256(abs(amount0), 18) / pow(10, 18) ) AS volume0, -- normalize to 18 decimals to fit as Float64

   -- swaps (inverse) --
   argMinState(1 / price, global_sequence) AS open1,
   quantileExact(0.95)(1 / price) AS high1,
   quantileExact(0.05)(1 / price) AS low1,
   argMaxState(1 / price, global_sequence) AS close1,
   sumState(toDecimal256(abs(amount1), 18) / pow(10, 18) ) AS volume1, -- normalize to 18 decimals to fit as Float64

   -- universal --
   uniqState(sender) AS uaw,
   sumState(1) AS transactions
FROM swaps
GROUP BY pool, timestamp;
