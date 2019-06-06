#![feature(proc_macro_hygiene, decl_macro, custom_attribute, type_alias_enum_variants)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate rocket_contrib;
extern crate bcrypt;
extern crate toml;
extern crate regex;
extern crate chrono;
extern crate frank_jwt;

pub mod msg;
pub mod cr;
pub mod env;
pub mod parser;
pub mod service;
pub mod db;
pub mod model;
pub mod schema;
pub mod current_user;
pub mod view_model;
mod app;

const DEFAULT_CONFIG_FILENAME: &str = "./env/dev.toml";
const ENV_SECRET_KEY: &str = "CYMER_SECRET";
const ENV_CYMER_ENV_KEY: &str = "CYMER_ENV";

fn hello_world(env: &env::Env) {
    println!("{} - v{} running on {} mode", env.app.name, env.app.version.to_string(),
        if env.app.prod { String::from("prod") } else { String::from("debug") });
}

fn check_env() -> Result<(), std::env::VarError> {
    std::env::var(ENV_SECRET_KEY)?;
    std::env::var(ENV_CYMER_ENV_KEY)?;
    Result::Ok(())
}

fn main() {
    let env = env::Env::load(get_config_path().as_str());
    hello_world(&env);
    check_env().unwrap();
    let rocket = rocket::ignite()
        .manage(db::init_pool(env.db.uri.clone()));
    app::routes(rocket)
        .mount("/public", rocket_contrib::serve::StaticFiles::from("static"))
        .launch();
}

fn get_config_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return String::from(DEFAULT_CONFIG_FILENAME);
    }
    return String::from(args[1].clone());
}
