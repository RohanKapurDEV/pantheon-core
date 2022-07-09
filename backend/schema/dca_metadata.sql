create table if not exists dca_metadata (
    dca_metadata_id serial primary key,
    network varchar(10) not null,
    inserted_at timestamp not null default now(),
    created_at varchar(100) not null,
    dca_metadata_address varchar(50) not null unique,
    owner_address varchar(50) not null,
    from_token_mint varchar(50) not null,
    to_token_mint varchar(50) not null,
    owner_from_token_account varchar(50) not null,
    owner_to_token_account varchar(50) not null,
    vault_from_token_account varchar(50) not null,
    vault_to_token_account varchar(50) not null,
    amount_per_interval bigint unsigned not null,
    interval_length bigint unsigned not null,
    interval_counter int unsigned not null,
    max_intervals int unsigned not null,
    crank_authority varchar(50) not null
);