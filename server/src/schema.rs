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
    precipitation_logs (id) {
        id -> Uuid,
        measurement -> Float4,
        logged_at -> Timestamptz,
        notes -> Nullable<Text>,
        ptype -> Int2,
        anomaly -> Bool,
        deleted -> Bool,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
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

allow_tables_to_appear_in_same_query!(
    api_tokens,
    precipitation_logs,
    users,
);
