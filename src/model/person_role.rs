use serde_derive::{Serialize, Deserialize};
use crate::schema::person_role;

#[derive(Debug, Serialize, Deserialize)]
pub enum PersonRoleEnum {
    Admin = 1,
    User = 2,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "person_role"]
pub struct PersonRole {
    pub id: i32,
    pub label: String,
}

pub fn to_enum(id: i32) -> PersonRoleEnum {
    match id {
        1 => PersonRoleEnum::Admin,
        2 => PersonRoleEnum::User,
        _ => unreachable!(),
    }
}
