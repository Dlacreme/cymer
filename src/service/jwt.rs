use frank_jwt::{Algorithm, encode, decode};
use frank_jwt::error::Error;

#[derive(Debug)]
pub struct Payload {
    people_id: i32,
    active_company_id: i32,
}

impl Payload {
    fn from_token(_header: serde_json::Value, payload: serde_json::Value) -> Self {
        Self {
            people_id: payload["people_id"].to_string().parse::<i32>().unwrap(),
            active_company_id: payload["active_company_id"].to_string().parse::<i32>().unwrap(),
        }
    }
}

pub fn serialize(payload: Payload) -> Result<String, Error> {
    let json_header = json!({});
    let json_payload = json!({
        "people_id": payload.people_id,
        "active_company_id": payload.active_company_id,
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
