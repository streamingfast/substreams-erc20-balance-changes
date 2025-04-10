-- Historical ERC-20 Balances by Address--
SELECT
      contract,
      timestamp,
      argMinMerge(open)  AS open,
      max(high)     AS high,
      min(low)      AS low,
      argMaxMerge(close) AS close,
      sumMerge(transactions) AS transactions
FROM historical_erc20_balances
WHERE
      address = '0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b'
GROUP BY address,contract,timestamp;

-- Unique Active Wallets & Transactions by ERC-20 Contracts --
SELECT
      contract,
      timestamp,
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM historical_erc20_balances_by_contract
GROUP by contract,timestamp;

-- Unique Active Wallets & Transactions by Native asset (ex: ETH/BNB) --
SELECT
      timestamp,
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM historical_native_balances
GROUP by timestamp;
