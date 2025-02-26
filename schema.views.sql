-- Latest balances by account
CREATE TABLE IF NOT EXISTS account_balances ON CLUSTER '{cluster}'
(
    contract            LowCardinality(String),
        
    account               String,
    balance             Float64,

    last_update_block   UInt64,
    last_update_time    DateTime,
    version             UInt64
)
    ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{uuid}/{shard}', '{replica}', version)
        ORDER BY (account, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS account_balances_mv ON CLUSTER '{cluster}'
    TO account_balances
AS
SELECT contract,
       owner AS account,
       amount AS balance,
       block_num AS last_update_height,
       timestamp AS last_update_time,
       version
FROM balance_changes;

-- Historical balances by account
CREATE TABLE IF NOT EXISTS historical_account_balances ON CLUSTER '{cluster}'
(
    contract      LowCardinality(String),

    account       String,
    balance       Float64,

    block_num     UInt64,
    timestamp     DateTime,
    version       UInt64
)
    ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{uuid}/{shard}', '{replica}', version)
        ORDER BY (block_num, account, contract);

CREATE MATERIALIZED VIEW IF NOT EXISTS historical_account_balances_mv ON CLUSTER '{cluster}'
    TO historical_account_balances
AS
SELECT contract,
       owner AS account,
       amount AS balance,
       block_num,
       timestamp,
       version
FROM balance_changes;

-- Token holders with positive balances by contract
CREATE TABLE IF NOT EXISTS token_holders ON CLUSTER '{cluster}'
(
    contract            LowCardinality(String),
        
    account             String,
    balance             Float64,

    last_update_height  UInt64,
    last_update_time    DateTime,
    version             UInt64
)
    ENGINE = ReplicatedReplacingMergeTree('/clickhouse/tables/{uuid}/{shard}', '{replica}', version)
        ORDER BY (contract, account);

-- Clean up null balances from the table on part merge
ALTER TABLE token_holders
    MODIFY TTL last_update_time WHERE balance <= 0;

CREATE MATERIALIZED VIEW IF NOT EXISTS token_holders_mv ON CLUSTER '{cluster}'
TO token_holders
AS
SELECT contract,
       owner AS account,
       amount AS balance,
       block_num AS last_update_height,
       timestamp AS last_update_time,
       version
FROM balance_changes;