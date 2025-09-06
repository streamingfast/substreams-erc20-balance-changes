-- Finalized read view over the AggregatingMergeTree "state" table
CREATE VIEW IF NOT EXISTS historical_total_supply AS
SELECT
    -- block/window
    timestamp,
    min(block_num)                       AS block_num,

    -- keys
    contract,

    -- OHLC finalized
    argMinMerge(open)                    AS open,
    max(high)                            AS high,
    min(low)                             AS low,
    argMaxMerge(close)                   AS close,

    -- activity finalized
    sum(transactions)                    AS transactions
FROM historical_total_supply_state
GROUP BY contract, timestamp;
