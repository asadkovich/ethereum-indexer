CREATE TABLE IF NOT EXISTS transactions (
    hash varchar(128) primary key,
    block_hash string,
    block_number integer,
    from varchar(128),
    to varchar(128),
    value varchar(128),
    ts timestamp  not null,
);