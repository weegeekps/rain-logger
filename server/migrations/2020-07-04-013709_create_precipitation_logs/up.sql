CREATE TABLE precipitation_logs
(
    id          uuid                     not null primary key,
    measurement decimal                  not null,
    logged_at   timestamp with time zone not null,
    notes       text,
    anomaly     boolean                  not null default false,
    deleted     boolean                  not null default false,
    created_at  timestamp with time zone not null default current_timestamp,
    modified_at timestamp with time zone not null default current_timestamp,
    type_id     uuid                     not null,
    FOREIGN KEY (type_id) REFERENCES precipitation_log_types (id)
)
