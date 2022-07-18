create table if not exists payment_schedule (
    payment_schedule_id serial primary key,
    network varchar(10) not null,
    inserted_at timestamp not null default now(),
    timestamp int unsigned not null,
    dca_metadata_id bigint unsigned not null,
    dca_metadata_address varchar(50) not null
)