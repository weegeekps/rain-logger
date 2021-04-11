CREATE TABLE precipitation_logs
(
    id          uuid                     not null primary key,
    measurement real                     not null,
    logged_at   timestamp with time zone not null,
    notes       text,
    ptype       smallint                 not null,
    anomaly     boolean                  not null default false,
    deleted     boolean                  not null default false,
    created_at  timestamp with time zone not null default current_timestamp,
    modified_at timestamp with time zone not null default current_timestamp
);
