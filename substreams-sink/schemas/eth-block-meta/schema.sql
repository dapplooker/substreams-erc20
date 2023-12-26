
CREATE SCHEMA stack_os_wallet;
CREATE TABLE stack_os_wallet.cursors 
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);

CREATE TABLE stack_os_wallet.ethereum_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);

CREATE TABLE stack_os_wallet.bnb_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);

CREATE TABLE stack_os_wallet.poygon_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);