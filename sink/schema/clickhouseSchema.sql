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

CREATE TABLE IF NOT EXISTS erc20.Token (
    name           String,
    decimals           String,
    symbol           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (name);

CREATE TABLE IF NOT EXISTS erc20.Account (
    balances           String,
) ENGINE = ReplacingMergeTree()
ORDER BY (balances);

CREATE TABLE IF NOT EXISTS erc20.Balance (  
    token           String,
    owner           String,
    balance           String
) ENGINE = ReplacingMergeTree()
ORDER BY (token);

