table! {
    employee (employee_id) {
        person_id -> Int4,
        employee_id -> Int4,
        company_id -> Int4,
        is_disabled -> Bool,
        employee_role_id -> Varchar,
    }
}

table! {
    available_company(id) {
        id -> Int4,
        label -> Varchar,
    }
}
