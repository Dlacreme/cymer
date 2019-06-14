pub mod jwt;
pub mod reset_password;

pub fn is_prod() -> bool {
    std::env::var(crate::ENV_CYMER_ENV_KEY).unwrap().to_string() == "PROD"
}
