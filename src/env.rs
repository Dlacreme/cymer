use serde_derive::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct Env  {
    pub app: App,
    pub db: Db,
    pub network: Network,
}

#[derive(Deserialize, Clone)]
pub struct App {
    pub version: f32,
    pub name: String,
    pub prod: bool,
}

#[derive(Deserialize, Clone)]
pub struct Db {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
    pub uri: String,
}

#[derive(Deserialize, Clone)]
pub struct Network {
    pub host: String,
    pub port: u16,
}

impl Env {

    pub fn load(path: &str) -> Env {
        match super::parser::toml::parse::<Env>(path) {
            Ok(conf) => conf,
            Err(err) => {
                println!("Could not parse config file ({})\nBuild default one.", err.to_string());
                Env::default()
            }
        }
    }

    fn default() -> Env {
        Env {
            app: App {
                version: 0.1,
                name: String::from("John Doe Application"),
                prod: false
            },
            db: Db {
                host: String::from("localhost"),
                port: 5432,
                name: String::from("cymer_dev"),
                user: String::from("cymer"),
                password: String::from("cymer_rules"),
                uri: String::from(""),
            },
            network: Network {
                host: String::from("localhost"),
                port: 7070,
            },
        }
    }

}
