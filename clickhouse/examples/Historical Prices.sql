-- OHLC Prices by Pool --
SELECT
      timestamp,
      'WETH/USDT' AS ticker,

      -- OHLC --
      floor(argMinMerge(open0) * pow(10, 18-6), 2)       AS open,
      floor(quantile(0.95)(high0) * pow(10, 18-6), 2)    AS high,
      floor(quantile(0.05)(low0) * pow(10, 18-6), 2)     AS low,
      floor(argMaxMerge(close0) * pow(10, 18-6), 2)      AS close,
      floor(sumMerge(volume0), 2)                        AS "volume (ETH)" -- volume is in wei, no need to convert it
FROM ohlc_prices
WHERE pool = '0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b' -- Uniswap V3 WETH/USDT
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
WHERE pool = '0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b' -- Uniswap V3 WETH/USDT
GROUP BY pool, date;