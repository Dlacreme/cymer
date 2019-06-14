use crate::db;
use diesel::PgConnection;
use frank_jwt::{encode, Algorithm};

const TOKEN_DURATION: i64 = 1;

pub fn invite(co: &PgConnection, person_id: i32) -> Result<(), ()> {
    process(co, person_id)
}

pub fn reset(co: &PgConnection, person_id: i32) -> Result<(), ()> {
    process(co, person_id)
}

fn process(co: &PgConnection, person_id: i32) -> Result<(), ()> {
    match db::person::clear_password(co, person_id) {
        Ok(_) => (),
        Err(_) => return Result::Err(()),
    };
    build_token(person_id, get_valid_until().timestamp())?;
    Result::Ok(())
}

fn build_token(person_id: i32, valid_until: i64) -> Result<String, ()> {
    let json_header = json!({
        "alg": "HS256",
        "typ": "JWT",
    });
    let json_payload = json!({
        "valid_until": valid_until,
        "person_id": person_id,
    });

    match encode(json_header, &get_secret(), &json_payload, Algorithm::HS256) {
        Ok(r) => Result::Ok(r),
        Err(_) => Result::Err(()),
    }
}

fn get_valid_until() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc() + chrono::Duration::days(TOKEN_DURATION)
}

fn get_secret() -> String {
    std::env::var(crate::ENV_SECRET_KEY).unwrap().to_string()
}
