table! {
    access (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    channel (id) {
        id -> Int4,
    }
}

table! {
    company (id) {
        id -> Int4,
        label -> Varchar,
        created_by_id -> Int8,
        created_at -> Timestamp,
        is_disabled -> Bool,
    }
}

table! {
    employee (id) {
        id -> Int4,
        user_id -> Int8,
        company_id -> Int4,
        employee_role_id -> Int2,
        is_disabled -> Bool,
    }
}

table! {
    employee_role (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    message (id) {
        id -> Int4,
        channel_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        content -> Nullable<Text>,
        created_at -> Timestamp,
        is_disabled -> Bool,
    }
}

table! {
    notification (id) {
        id -> Int4,
        channel_id -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        key -> Varchar,
        data -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    people (id) {
        id -> Int4,
        role_id -> Int2,
        email -> Varchar,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        profile_id -> Int4,
        active_company_id -> Nullable<Int4>,
        notif_counter -> Int2,
    }
}

table! {
    profile (id) {
        id -> Int4,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
    }
}

table! {
    role (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        role_id -> Int2,
        email -> Varchar,
        password -> Nullable<Varchar>,
        provider_id -> Nullable<Varchar>,
        provider_type -> Nullable<Varchar>,
        created_at -> Timestamp,
        profile_id -> Nullable<Int8>,
        active_company_id -> Nullable<Int4>,
        notif_counter -> Int2,
        chat_counter -> Int2,
    }
}

joinable!(employee -> company (company_id));
joinable!(employee -> employee_role (employee_role_id));
joinable!(employee -> user (user_id));
joinable!(message -> channel (channel_id));
joinable!(message -> user (user_id));
joinable!(notification -> channel (channel_id));
joinable!(user -> profile (profile_id));
joinable!(user -> role (role_id));

allow_tables_to_appear_in_same_query!(
    access,
    channel,
    company,
    employee,
    employee_role,
    message,
    notification,
    people,
    profile,
    role,
    user,
);
