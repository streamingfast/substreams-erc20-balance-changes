-- OHLC prices including Uniswap V2 & V3 --
CREATE TABLE IF NOT EXISTS ohlc_prices (
   -- block --
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
   pool,

   -- swaps --
   argMinState(price, global_sequence) AS open0,
   quantileDeterministic(0.95)(price, global_sequence) AS high0,
   quantileDeterministic(0.05)(price, global_sequence) AS low0,
   argMaxState(price, global_sequence) AS close0,
   sumState(toDecimal256(amount0, 18) * price ) AS volume0,

   -- swaps (inverse) --
   argMinState(1 / price, global_sequence) AS open1,
   quantileDeterministic(0.95)(1 / price, global_sequence) AS high1,
   quantileDeterministic(0.05)(1 / price, global_sequence) AS low1,
   argMaxState(1 / price, global_sequence) AS close1,
   sumState(toDecimal256(amount0, 18) * (1 / price) ) AS volume1,

   -- universal --
   uniqState(sender) AS uaw,
   sumState(1) AS transactions
FROM swaps
GROUP BY pool, timestamp;
