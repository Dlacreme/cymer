use regex::Regex;
use super::regex_model;

static INVALID_EMAIL: &str = "Invalid email type";
static INVALID_PASSWORD: &str = "Password contains invalid characted or has less than 8 characters";

pub fn is_email(email: &str) -> Result<(), String> {
    test_regex(regex_model::EMAIL, email, INVALID_EMAIL)
}

pub fn is_valid_password(password: &str) -> Result<(), String> {
    test_regex(regex_model::PASSWORD, password, INVALID_PASSWORD)
}

fn test_regex(model: &str, value: &str, message: &str) -> Result<(), String> {
    if Regex::new(model).unwrap().is_match(value) == false {
        return Result::Err(String::from(message));
    }
    Result::Ok(())
}
