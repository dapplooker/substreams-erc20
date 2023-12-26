
CREATE SCHEMA stack_os_data;
CREATE TABLE stack_os_data.cursors 
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);

CREATE TABLE stack_os_data.ethereum_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);

CREATE TABLE stack_os_data.bnb_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);

CREATE TABLE stack_os_data.polygon_wallet_balance (
    owner_address text,
    balance numeric,
    transaction_hash text,
    creation_time text
);