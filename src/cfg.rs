use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
}

use std::{
    io::Read,
    fs::OpenOptions,
};

impl Config {
    pub fn get(file_name: &str) -> Self {
        let mut cfg_dir = dirs::config_dir().unwrap();
        cfg_dir.push(file_name);

        let mut buf = String::new();
        let mut file = OpenOptions::new()
                .read(true).write(true).create(true)
                .open(cfg_dir).unwrap();
        file.read_to_string(&mut buf).unwrap();


        toml::from_str(&buf).unwrap()
    }
}
