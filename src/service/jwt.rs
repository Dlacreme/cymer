use frank_jwt::{Algorithm, encode, decode};

pub struct Payload {
    people_id: i32,
    active_company_id: i32,
}

pub fn serialize(payload: Payload) {
    let mut header = json!({});
    let secret = std::env::var(crate::ENV_SECRET_KEY).unwrap();
    let jwt = encode(&header, secret.to_string(), &payload, Algorithm::HS256);
}