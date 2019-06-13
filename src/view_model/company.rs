use crate::db;
use crate::model::company::Company as CompanyModel;
use diesel::{PgConnection, QueryResult};
use serde_derive::{Deserialize, Serialize};

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
    pub fn from_model(co: &CompanyModel) -> Self {
        Self {
            id: co.id,
            label: co.label.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyEmployees {
    pub id: i32,
    pub label: String,
    pub employees: Vec<super::employee::Employee>,
}

impl CompanyEmployees {
    pub fn from_db(co: &PgConnection, id: i32) -> QueryResult<Self> {
        let company = db::company::find(co, id)?;
        let emps = super::employee::Employee::list_from_db(co, id)?;
        QueryResult::Ok(Self {
            id: company.id,
            label: company.label,
            employees: emps,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyToCreate {
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyToUpdate {
    pub label: Option<String>,
}
