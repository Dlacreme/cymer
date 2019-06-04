use frank_jwt::{Algorithm, encode, decode};
use frank_jwt::error::Error;
use crate::model::person::Person;

#[derive(Debug)]
pub struct Payload {
    people_id: i32,
    active_company_id: Option<i32>,
}

impl Payload {
    pub fn from_person(person: &Person) -> Self {
        Self {
            people_id: person.id,
            active_company_id: person.active_company_id,
        }
    }

    fn from_token(_header: serde_json::Value, payload: serde_json::Value) -> Self {
        // If current_company_id = 0, we set its value to None
        let company_id = payload["active_company_id"].to_string().parse::<i32>().unwrap();
        Self {
            people_id: payload["people_id"].to_string().parse::<i32>().unwrap(),
            active_company_id: match company_id > 0 {
                true => Some(company_id),
                false => None,
            },
        }
    }
}

pub fn serialize(payload: Payload) -> Result<String, Error> {
    let json_header = json!({});
    let json_payload = json!({
        "people_id": payload.people_id,
        "active_company_id": payload.active_company_id.unwrap_or(0), // 0 will be converted to None
    });
    encode(json_header, &get_secret(), &json_payload, Algorithm::HS256)
}

pub fn deserialize(jwt: String) -> Result<Payload, Error> {
    let (header, payload) = decode(&jwt, &get_secret(), Algorithm::HS256)?;
    Result::Ok(Payload::from_token(header, payload))
}

fn get_secret() -> String {
    std::env::var(crate::ENV_SECRET_KEY).unwrap().to_string()
}
