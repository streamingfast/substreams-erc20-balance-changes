WITH tokens AS (
    SELECT lower('dac17f958d2ee523a2206206994597c13d831ec7') AS contract, 'USDT' AS symbol
    UNION ALL SELECT lower('a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'), 'USDC'
    UNION ALL SELECT lower('c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2'), 'WETH'
    UNION ALL SELECT lower('b8c77482e45f1f44de1745f52c74426c631bdd52'), 'BNB'
    UNION ALL SELECT lower('4fabb145d64652a948d72533023f6e7a623c7c53'), 'BUSD'
    UNION ALL SELECT lower('2b591e99afe9f32eaa6214f7b7629768c40eeb39'), 'HEX'
    UNION ALL SELECT lower('95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce'), 'SHIB'
    UNION ALL SELECT lower('75231f58b43240c9718dd58b4967c5114342a86c'), 'OKB'
    UNION ALL SELECT lower('2af5d2ad76741191d15dfe7bf6ac92d4bd912ca3'), 'LEO'         -- ❌ in original comment
    UNION ALL SELECT lower('1f9840a85d5aF5bf1D1762F925BDADdC4201F984'), 'UNI'
    UNION ALL SELECT lower('514910771af9ca656af840dff83e8264ecf986ca'), 'LINK'
    UNION ALL SELECT lower('6b175474e89094c44da98b954eedeac495271d0f'), 'DAI'
    UNION ALL SELECT lower('9f8F72aA9304c8B593d555F12eF6589cC3A579A2'), 'MKR (Maker)'
    UNION ALL SELECT lower('2260fac5e5542a773aa44fbcfedf7c193bc2c599'), 'WBTC (Wrapped Bitcoin)'
    UNION ALL SELECT lower('7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0'), 'MATIC'
    UNION ALL SELECT lower('1456688345527be1f37e9e627da0837d6f08c925'), 'USDP'
    UNION ALL SELECT lower('0000000000085d4780b73119b644ae5ecd22b376'), 'TUSD'
    UNION ALL SELECT lower('4d224452801aced8b2f0aebe155379bb5d594381'), 'APE'
    UNION ALL SELECT lower('b47e3cd837ddf8e4c57f05d70ab865de6e193bbb'), 'PUNK'
    UNION ALL SELECT lower('0f5d2fb29fb7d3cfee444a200298f468908cc942'), 'MANA'
    UNION ALL SELECT lower('3845badade8e6dff049820680d1f14bd3903a5d0'), 'SAND'
    UNION ALL SELECT lower('f629cbd94d3791c9250152bd8dfbdf380e2a3b9c'), 'ENJ'
    UNION ALL SELECT lower('bbbbca6a901c926f240b89eacb641d8aec7aeafd'), 'LRC'
    UNION ALL SELECT lower('0abdace70d3790235af448c88547603b945604ea'), 'DNT'         -- ❌ in original comment
    UNION ALL SELECT lower('111111111117dC0aa78b770fA6A738034120C302'), '1INCH'
    UNION ALL SELECT lower('ae7ab96520de3a18e5e111b5eaab095312d7fe84'), 'stETH'       -- ❌ in original comment
    UNION ALL SELECT lower('c944e90c64b2c07662a292be6244bdf05cda44a7'), 'GRT'
    UNION ALL SELECT lower('7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9'), 'AAVE'        -- ❌ in original comment
    UNION ALL SELECT lower('152649eA73beAb28c5b49B26eb48f7EAD6d4c898'), 'CAKE'
    UNION ALL SELECT lower('5a98fcbea516cf06857215779fd812ca3bef1b32'), 'LDO'         -- ❌ in original comment
    UNION ALL SELECT lower('853d955acef822db058eb8505911ed77f175b99e'), 'FRAX'
    UNION ALL SELECT lower('bbbbca6a901c926f240b89eacb641d8aec7aeafd'), 'LRC (Loopring)'
    UNION ALL SELECT lower('0b38210ea11411557c13457d4da7dc6ea731b88a'), 'API3'
    UNION ALL SELECT lower('ba100000625a3754423978a60c9317c58a424e3d'), 'BAL'
    -- UNION ALL SELECT lower('ae78736cd615f374d3085123a210448e74fc6393'), 'rETH'
    UNION ALL SELECT lower('58b6a8a3302369daec383334672404ee733ab239'), 'LPT'
    UNION ALL SELECT lower('0bc529c00c6401aef6d220be8c6ea1667f6ad93e'), 'YFI'
    UNION ALL SELECT lower('d533a949740bb3306d119cc777fa900ba034cd52'), 'CRV'         -- ❌ in original comment
    -- UNION ALL SELECT lower('be9895146f7af43049ca1c1ae358b0541ea49704'), 'cbETH'
    UNION ALL SELECT lower('c00e94cb662c3520282e6f5717214004a7f26888'), 'COMP'
    -- Duplicate address for POLY: rename symbol to avoid confusion
    UNION ALL SELECT lower('455e53CBB86018Ac2B8092FdCd39d8444aFFC3F6'), 'POL (Polygon Ecosystem Token)'
    -- UNION ALL SELECT lower('f650c3d88cc8615fc5d0e846529a0978fee9a637'), 'cUSDT'
    -- UNION ALL SELECT lower('56d811088235f11c8920698a204a5010a788f4b3'), 'BZRX'        -- ❌ in original comment
    UNION ALL SELECT lower('408e41876cccdc0f92210600ef50372656052a38'), 'REN'
)

SELECT
    tk.contract,
    tk.symbol,
    -- Count how many transfers had no matching balance change
    COUNT(*) FILTER (WHERE t.contract IS NOT NULL AND b.transaction_id IS NULL) AS missed_transfers,

    -- Count how many transfers *did* match a balance change
    COUNT(*) FILTER (WHERE t.contract IS NOT NULL AND b.transaction_id IS NOT NULL) AS matched_transfers,

    -- Total transfers for that contract
    COUNT(t.transaction_id) AS total_transfers,

FROM tokens AS tk
LEFT JOIN read_parquet('./out/transfers/*.parquet') t
       ON tk.contract = t.contract
LEFT JOIN read_parquet('./out/balance_changes/*.parquet') b
       ON t.transaction_id = b.transaction_id
      AND t.log_index = b.log_index

GROUP BY tk.contract, tk.symbol
ORDER BY missed_transfers DESC;
