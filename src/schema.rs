table! {
    access (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    company (id) {
        id -> Int4,
        label -> Varchar,
        created_by_id -> Int4,
        created_at -> Timestamp,
        is_disabled -> Bool,
    }
}

table! {
    employee (id) {
        id -> Int4,
        person_id -> Int4,
        company_id -> Int4,
        employee_access_id -> Int4,
        is_disabled -> Bool,
    }
}

table! {
    employee_access (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    person (id) {
        id -> Int4,
        access_id -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        person_profile_id -> Int4,
        active_company_id -> Nullable<Int4>,
        notif_counter -> Int4,
    }
}

table! {
    person_profile (id) {
        id -> Int4,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
    }
}

joinable!(employee -> company (company_id));
joinable!(employee -> employee_access (employee_access_id));
joinable!(employee -> person (person_id));
joinable!(person -> access (access_id));
joinable!(person -> person_profile (person_profile_id));

allow_tables_to_appear_in_same_query!(
    access,
    company,
    employee,
    employee_access,
    person,
    person_profile,
);
