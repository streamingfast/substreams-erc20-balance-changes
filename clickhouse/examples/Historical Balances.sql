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
      address = '0x000000fee13a103a10d593b9ae06b3e05f2e7e1c' AND
      contract = '0x6fa7760abf096d94f9b330d96c05615991fb9026'
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
