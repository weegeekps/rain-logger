-- Creates the users table.
CREATE TABLE users
(
    id          uuid                     not null primary key,
    name        varchar                  not null,
    password    varchar                  not null,
    enabled     boolean                  not null default true,
    created_at  timestamp with time zone not null default current_timestamp,
    modified_at timestamp with time zone not null default current_timestamp
)
