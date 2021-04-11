CREATE TABLE precipitation_log_types
(
    id          uuid                     not null primary key,
    type        integer                  not null,
    created_at  timestamp with time zone not null default current_timestamp,
    modified_at timestamp with time zone not null default current_timestamp
)
