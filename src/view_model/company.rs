use crate::db;
use crate::model::company::Company as CompanyModel;
use crate::schema_manual::available_company;
use diesel;
use diesel::prelude::*;
use diesel::{sql_query, PgConnection, QueryResult};
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

#[derive(Queryable, QueryableByName, Serialize, Deserialize, Debug)]
#[table_name = "available_company"]
pub struct AvailableCompany {
    id: i32,
    label: String,
}

impl AvailableCompany {
    pub fn list_from_db(co: &PgConnection, person_id: i32) -> QueryResult<Vec<Self>> {
        sql_query(format!(
            "SELECT c.id, c.label       \
             FROM  employee e           \
             INNER JOIN company c       \
             ON c.id = e.company_id     \
             WHERE e.person_id = {}     \
             AND e.is_disabled = 'f'    \
             GROUP BY c.id              \
             ",
            person_id
        ))
        .load(co)
    }
}
