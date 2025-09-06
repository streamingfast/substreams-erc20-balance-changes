-- Historical ERC-20 total supply by contract --
CREATE TABLE IF NOT EXISTS historical_total_supply_state (
    -- block --
    timestamp    DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',
    block_num    SimpleAggregateFunction(min, UInt32) COMMENT 'the minimum block number in the aggregate window',

    -- total supply --
    contract     String,

    -- ohlc --
    open         AggregateFunction(argMin, UInt256, UInt32),
    high         SimpleAggregateFunction(max, UInt256),
    low          SimpleAggregateFunction(min, UInt256),
    close        AggregateFunction(argMax, UInt256, UInt32),
    transactions SimpleAggregateFunction(sum, UInt64) COMMENT 'total number of supply changes in the window'
)
ENGINE = AggregatingMergeTree
ORDER BY (contract, timestamp);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_historical_total_supply
TO historical_total_supply_state AS
SELECT
    -- block --
    toStartOfHour(timestamp) AS timestamp,
    min(t.block_num) AS block_num,

    -- total supply --
    contract,

    -- ohlc --
    argMinState(total_supply, t.block_num) AS open,
    max(total_supply) AS high,
    min(total_supply) AS low,
    argMaxState(total_supply, t.block_num) AS close,
    count() AS transactions
FROM total_supply as t
GROUP BY contract, timestamp;
