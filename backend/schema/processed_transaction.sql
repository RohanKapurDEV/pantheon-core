create table if not exists ProcessedSchedule (
    processed_schedule_id serial primary key,
    network varchar(10) not null,
    inserted_at timestamp not null default now(),
    tx_sig varchar(150) not null,
    dca_metadata_id bigint unsigned not null,
    dca_metadata_address varchar(50) not null
);