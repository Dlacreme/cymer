use frank_jwt::{Algorithm, encode, decode};
use frank_jwt::error::Error;
use crate::model::person::Person;

const TOKEN_DURATION: i64 = 30;

#[derive(Debug)]
pub struct Payload {
    pub person_id: i32,
    pub person_role_id: i32,
    pub active_company_id: Option<i32>,
}

impl Payload {
    pub fn from_person(person: &Person) -> Self {
        Self {
            person_id: person.id,
            person_role_id: person.person_role_id,
            active_company_id: person.active_company_id,
        }
    }

    fn from_token(_header: serde_json::Value, payload: serde_json::Value) -> Option<Self> {
        // If current_company_id = 0, we set its value to None
        let company_id: i32 = payload["active_company_id"].as_i64()? as i32;
        Some(Self {
            person_id: payload["person_id"].as_i64()? as i32,
            person_role_id: payload["person_role_id"].as_i64()? as i32,
            active_company_id: match company_id > 0 {
                true => Some(company_id),
                false => None,
            },
        })
    }
}

pub fn serialize(payload: Payload) -> Result<String, Error> {
    let json_header = json!({
        "alg": "HS256",
        "typ": "JWT",
    });
    let json_payload = json!({
        "valid_until": get_valid_until().timestamp(),
        "person_id": payload.person_id,
        "person_role_id": payload.person_role_id,
        "active_company_id": payload.active_company_id.unwrap_or(0), // 0 will be converted to None
    });
    encode(json_header, &get_secret(), &json_payload, Algorithm::HS256)
}

pub fn deserialize(jwt: String) -> Result<Payload, Error> {
    let (header, payload) = decode(&jwt, &get_secret(), Algorithm::HS256)?;
    if chrono::NaiveDateTime::from_timestamp(payload["valid_until"].as_i64().unwrap(), 0) < chrono::Utc::now().naive_utc() {
        return Result::Err(Error::ExpirationInvalid);
    }
    match Payload::from_token(header, payload) {
        Some(token) => Result::Ok(token),
        None => Result::Err(Error::FormatInvalid(String::from("INVALID TOKEN FORMAT"))),
    }
}

fn get_secret() -> String {
    std::env::var(crate::ENV_SECRET_KEY).unwrap().to_string()
}

fn get_valid_until() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc() + chrono::Duration::days(TOKEN_DURATION)
}
