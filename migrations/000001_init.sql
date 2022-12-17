CREATE TABLE IF NOT EXISTS transactions (
    hash varchar(128) primary key,
    block_hash string,
    block_number integer,
    from varchar(128),
    to varchar(128),
    value varchar(128),
    ts timestamp  not null,
    UNIQUE (hash)
);

CREATE TABLE IF NOT EXISTS blocks (
    hash varchar(128) primary key,
    number integer,
    parent_hash varchar(128),
    nonce varchar(128),
    sha3_uncles varchar(128),
    logs_bloom varchar(128),
    transactions_root varchar(128),
    state_root varchar(128),
    receipts_root varchar(128),
    miner varchar(128),
    difficulty varchar(128),
    total_difficulty varchar(128),
    extra_data varchar(128),
    size integer,
    gas_limit integer,
    gas_used integer,
    timestamp timestamp not null,
    ts timestamp not null,
    UNIQUE (hash)
);