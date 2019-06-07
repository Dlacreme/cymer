table! {
    company (id) {
        id -> Int4,
        label -> Varchar,
        created_by_id -> Int4,
        is_disabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    employee (id) {
        id -> Int4,
        person_id -> Int4,
        company_id -> Int4,
        is_disabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    employee_access (employee_id, employee_role_id) {
        employee_id -> Int4,
        employee_role_id -> Int4,
    }
}

table! {
    employee_role (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    person (id) {
        id -> Int4,
        person_role_id -> Int4,
        email -> Varchar,
        password -> Varchar,
        person_profile_id -> Int4,
        active_company_id -> Nullable<Int4>,
        notif_counter -> Int4,
        is_disabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    person_profile (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    person_role (id) {
        id -> Int4,
        label -> Varchar,
    }
}

joinable!(employee -> company (company_id));
joinable!(employee -> person (person_id));
joinable!(employee_access -> employee (employee_id));
joinable!(employee_access -> employee_role (employee_role_id));
joinable!(person -> person_profile (person_profile_id));
joinable!(person -> person_role (person_role_id));

allow_tables_to_appear_in_same_query!(
    company,
    employee,
    employee_access,
    employee_role,
    person,
    person_profile,
    person_role,
);
