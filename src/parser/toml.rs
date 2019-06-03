use std::fs::File;
use std::io::prelude::*;

pub fn parse<T>(path: &str) -> Result<T, String>
where T: serde::de::DeserializeOwned {
    let mut file = File::open(path).map_err(|e| format!("Could not open file {} > {}", path, e.to_string()))?;
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).map_err(|e| format!("Couldnt not read file {} > {}", path, e.to_string()))?;
    let f: T = toml::from_str(&toml_str).map_err(|e| format!("Coudnt parse file {} > {}", path, e.to_string()))?;
    Ok(f)
}
