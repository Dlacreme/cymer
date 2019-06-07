use serde_derive::{Serialize, Deserialize};
use diesel::{PgConnection, QueryResult};
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: i32,
    pub label: String,
}

impl Company {
    pub fn from_db(conn: &PgConnection, id: i32) -> QueryResult<Self> {
        let company = db::company::find(conn, id)?;
        QueryResult::Ok(Self {
            id: company.id,
            label: company.label,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyToUpdate {
    label: String,
}