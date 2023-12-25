
CREATE SCHEMA stack_os_wallet;
CREATE TABLE stack_os_wallet.cursors 
(
    id         text not null constraint cursor_pk primary key,
    cursor     text,
    block_num  bigint,
    block_id   text
);

CREATE TABLE stack_os_wallet.wallet_balance (
    owner_address text not null constraint owner_address_pk primary key,
    balance numeric,
    transaction_hash text
);