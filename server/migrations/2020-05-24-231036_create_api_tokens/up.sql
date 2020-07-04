CREATE TABLE api_tokens
(
    id            uuid                     not null primary key,
    token         text                     not null,
    force_invalid boolean                  not null default false,
    created_at    timestamp with time zone not null default current_timestamp,
    modified_at   timestamp with time zone not null default current_timestamp,
    user_id       uuid                     not null,
    FOREIGN KEY (user_id) REFERENCES users (id)
)
