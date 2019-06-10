use super::regex_model;
use regex::Regex;

static INVALID_EMAIL: &str = "Invalid email format";
static INVALID_PASSWORD: &str = "Password contains invalid characted or has less than 8 characters";

pub fn is_email(email: &str) -> Result<(), &str> {
    test_regex(regex_model::EMAIL, email, INVALID_EMAIL)
}

pub fn is_valid_password(password: &str) -> Result<(), &str> {
    test_regex(regex_model::PASSWORD, password, INVALID_PASSWORD)
}

pub fn is_alpha(s: &str) -> Result<(), &str> {
    test_regex(regex_model::ALPHA, s, INVALID_PASSWORD)
}

pub fn is_num(s: &str) -> Result<(), &str> {
    test_regex(regex_model::NUM, s, INVALID_PASSWORD)
}

pub fn is_alphanum(s: &str) -> Result<(), &str> {
    test_regex(regex_model::ALPHANUM, s, INVALID_PASSWORD)
}

fn test_regex(model: &str, value: &str, message: &'static str) -> Result<(), &'static str> {
    if Regex::new(model).unwrap().is_match(value) == false {
        return Result::Err(message);
    }
    Result::Ok(())
}
