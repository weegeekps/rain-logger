table! {
    api_tokens (id) {
        id -> Uuid,
        force_invalid -> Bool,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        user_id -> Uuid,
    }
}

table! {
    precipitation_log_types (id) {
        id -> Uuid,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

table! {
    precipitation_logs (id) {
        id -> Uuid,
        measurement -> Numeric,
        logged_at -> Timestamptz,
        notes -> Nullable<Text>,
        anomaly -> Bool,
        deleted -> Bool,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        type_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        password -> Varchar,
        enabled -> Bool,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
    }
}

joinable!(api_tokens -> users (user_id));
joinable!(precipitation_logs -> precipitation_log_types (type_id));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    precipitation_log_types,
    precipitation_logs,
    users,
);
