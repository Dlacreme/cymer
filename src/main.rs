#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate serde;
extern crate bcrypt;
extern crate toml;
extern crate regex;
extern crate chrono;

pub mod output;
pub mod env;
pub mod parser;
pub mod service;
pub mod db;
pub mod model;
pub mod schema;
mod app;

const DEFAULT_CONFIG_FILENAME: &str = "./env/dev.toml";
fn hello_world(env: &env::Env) {
    println!("{} - v{} running on {} mode", env.app.name, env.app.version.to_string(),
        if env.app.prod { String::from("prod") } else { String::from("debug") });
}

fn main() {
    // Load and parse the env file
    let env = env::Env::load(get_config_path().as_str());
    hello_world(&env);
    let rocket = rocket::ignite()
        .manage(db::init_pool());
    app::routes(rocket)
        .launch();
}

fn get_config_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return String::from(DEFAULT_CONFIG_FILENAME);
    }
    return String::from(args[1].clone());
}
