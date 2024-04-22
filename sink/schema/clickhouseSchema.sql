-- Create schema
CREATE DATABASE IF NOT EXISTS erc20;

-- Create cursors table
CREATE TABLE IF NOT EXISTS erc20.cursors (
    id        String NOT NULL,
    cursor    String,
    block_num Int64,
    block_id  String
) ENGINE = ReplacingMergeTree()
    PRIMARY KEY (id)
    ORDER BY (id);

CREATE TABLE IF NOT EXISTS erc20.token (
    name           String,
    decimals           String,
    symbol           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (name);

CREATE TABLE IF NOT EXISTS erc20.account (
    account           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (account);

CREATE TABLE IF NOT EXISTS erc20.balance (  
    token           FixedString(85),
    owner           FixedString(42),
    balance         String
) ENGINE = ReplacingMergeTree()
ORDER BY (token);

