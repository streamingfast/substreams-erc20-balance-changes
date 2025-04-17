-- ERC-721 Transfers Table
CREATE TABLE IF NOT EXISTS transfers (
    block_num        UInt64,
    tx_hash          FixedString(66),
    log_index        UInt64,
    contract         FixedString(42),
    from             FixedString(42),
    to               FixedString(42),
    token_id         String
    uri              Nullable(String)
    symbol           Nullable(String)
    name             Nullable(String)
) ENGINE = ReplacingMergeTree
PRIMARY KEY (block_num, tx_hash, log_index)
ORDER BY (block_num, tx_hash, log_index);

-- ERC-721 Transactions Table
CREATE TABLE IF NOT EXISTS transactions (
    block_number             UInt64,
    block_timestamp          UInt64,
    block_hash               FixedString(66),
    tx_hash                  FixedString(66),
    nonce                    UInt64,
    index                 UInt32,
    from_address             FixedString(42),
    to_address               FixedString(42),
    value                    String,
    tx_fee                   String,
    gas_price                String,
    gas_limit                UInt64,
    gas_used                 UInt64,
    cumulative_gas_used      UInt64,
    max_fee_per_gas          String,
    max_priority_fee_per_gas String,
    input                    String,
    type                     Int32,
    v                        String,
    r                        String,
    s                        String
) ENGINE = ReplacingMergeTree
PRIMARY KEY (block_number, tx_hash)
ORDER BY (block_number, tx_hash);
