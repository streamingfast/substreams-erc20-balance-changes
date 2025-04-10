-- OHLC Prices by Pool (Uniswap V3 WETH/USDT) --
SELECT
      timestamp,
      'WETH/USDT' AS ticker,

      -- -- OHLC --
      floor(argMinMerge(open0) * pow(10, 18-6), 2)       AS open,
      floor(quantileDeterministicMerge(0.99)(high0) * pow(10, 18-6), 2)    AS high,
      floor(quantileDeterministicMerge(0.01)(low0) * pow(10, 18-6), 2)     AS low,
      floor(argMaxMerge(close0) * pow(10, 18-6), 2)      AS close,

      -- volume --
      floor(sum(gross_volume0 / 10e18), 2)                    AS "gross volume (ETH)", -- volume is in wei, no need to convert it
      floor(sum(gross_volume1 / 10e6))                   AS "gross volume (USDT)",
      floor(sum(net_flow0 / 10e18), 2)                        AS "net flow (ETH)", -- volume is in wei, no need to convert it
      floor(sum(net_flow1 / 10e6))           AS "net flow (USDT)",

      -- universal --
      uniqMerge(uaw)                                  AS wallets,
      sum(transactions)                               AS tx_count
FROM ohlc_prices
WHERE pool = lower('0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b') -- Uniswap V3 WETH/USDT
GROUP BY pool, timestamp
ORDER BY timestamp DESC
LIMIT 10;

-- 24h Volume & Fees by Pool --
SELECT
      toDate(timestamp) as date,
      'WETH/USDT' AS ticker,

      -- Volume in USDT --
      floor(sumMerge(volume1) * pow(10, 18-6), 2)        AS "volume (USDT)", -- volume is in wei, so we need to convert it to USDT precision 6
      floor("volume (USDT)" * 100 / 1000000, 2)              AS fee, -- Uniswap V3 fee 0.01% (1=basis point)

      -- universal --
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM ohlc_prices
WHERE pool = lower('0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b') -- Uniswap V3 WETH/USDT
GROUP BY pool, date;

-- OHLC Prices by Pool (Uniswap V3 DAI/USDC) --
SELECT
      timestamp,
      'DAI/USDC' AS ticker,

      -- OHLC --
      floor(argMinMerge(open0) * pow(10, 18-6), 8)       AS open,
      floor(quantileDeterministicMerge(0.99)(high0) * pow(10, 18-6), 8)    AS high,
      floor(quantileDeterministicMerge(0.01)(low0) * pow(10, 18-6), 8)     AS low,
      floor(argMaxMerge(close0) * pow(10, 18-6), 8)      AS close,
      floor(sumMerge(volume0))                           AS "volume (DAI)" -- volume is in wei, no need to convert it
FROM ohlc_prices
WHERE pool = lower('0x5777d92f208679DB4b9778590Fa3CAB3aC9e2168') -- Uniswap V3 DAI/USDC
GROUP BY pool, timestamp
ORDER BY timestamp DESC;

-- OHLC Prices by Pool (Uniswap V3 WBTC/USDC) --
SELECT
      timestamp,
      'WBTC/USDC' AS ticker,

      -- OHLC --
      floor(argMinMerge(open0) * pow(10, 8-6), 2)       AS open,
      floor(quantileDeterministicMerge(0.99)(high0) * pow(10, 8-6), 2)    AS high,
      floor(quantileDeterministicMerge(0.01)(low0) * pow(10, 8-6), 2)     AS low,
      floor(argMaxMerge(close0) * pow(10, 8-6), 2)      AS close,
      floor(sumMerge(volume0) * pow(10, 18-8), 3)       AS "volume (WBTC)",
      floor(sumMerge(volume1) * pow(10, 18-6))          AS "volume (USDC)"
FROM ohlc_prices
WHERE pool = lower('0x99ac8cA7087fA4A2A1FB6357269965A2014ABc35') -- Uniswap V3 WBTC/USDC
GROUP BY pool, timestamp
ORDER BY timestamp DESC
LIMIT 20;