-- OHLC prices including Uniswaps with faster quantile computation --
CREATE TABLE IF NOT EXISTS ohlc_prices (
    timestamp            DateTime(0, 'UTC') COMMENT 'beginning of the bar',

    -- pool --
    pool                 String COMMENT 'pool address',
    protocol             SimpleAggregateFunction(any, LowCardinality(String)),

    -- token0 erc20 metadata --
    token0               SimpleAggregateFunction(any, FixedString(42)),
    decimals0            SimpleAggregateFunction(any, UInt8),
    symbol0              SimpleAggregateFunction(anyLast, Nullable(String)),
    name0                SimpleAggregateFunction(anyLast, Nullable(String)),

    -- token1 erc20 metadata --
    token1               SimpleAggregateFunction(any, FixedString(42)),
    decimals1            SimpleAggregateFunction(any, UInt8),
    symbol1              SimpleAggregateFunction(anyLast, Nullable(String)),
    name1                SimpleAggregateFunction(anyLast, Nullable(String)),

    -- canonical pair (token0, token1) lexicographic order --
    canonical0           SimpleAggregateFunction(any, FixedString(42)),
    canonical1           SimpleAggregateFunction(any, FixedString(42)),

    -- swaps --
    open0                AggregateFunction(argMin, Float64, UInt64),
    quantile0            AggregateFunction(quantileDeterministic, Float64, UInt64),
    close0               AggregateFunction(argMax, Float64, UInt64),

    -- volume --
    gross_volume0        SimpleAggregateFunction(sum, Float64) COMMENT 'gross volume of token0 in the window',
    gross_volume1        SimpleAggregateFunction(sum, Float64) COMMENT 'gross volume of token1 in the window',
    net_flow0            SimpleAggregateFunction(sum, Float64) COMMENT 'net flow of token0 in the window',
    net_flow1            SimpleAggregateFunction(sum, Float64) COMMENT 'net flow of token1 in the window',

    -- universal --
    uaw                  AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses in the window',
    transactions         SimpleAggregateFunction(sum, UInt64) COMMENT 'number of transactions in the window',

    -- indexes --
    INDEX idx_protocol          (protocol)                  TYPE set(4)         GRANULARITY 4,
    INDEX idx_token0            (token0)                    TYPE set(64)        GRANULARITY 4,
    INDEX idx_token1            (token1)                    TYPE set(64)        GRANULARITY 4,

    -- indexes (volume) --
    INDEX idx_gross_volume0     (gross_volume0)             TYPE minmax         GRANULARITY 4,
    INDEX idx_gross_volume1     (gross_volume1)             TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow0         (net_flow0)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow1         (net_flow1)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_transactions      (transactions)              TYPE minmax         GRANULARITY 4,

    -- indexes (canonical pair) --
    INDEX idx_canonical_pair    (canonical0, canonical1)    TYPE set(64)        GRANULARITY 4,
    INDEX idx_canonical_pair0   (canonical0)                TYPE set(64)        GRANULARITY 4,
    INDEX idx_canonical_pair1   (canonical1)                TYPE set(64)        GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (pool, timestamp);
