use std::{
    fs::{self, File},
    io::{Read, Write},
};

use log::warn;
use serde::{Deserialize, Serialize};
use sys_locale::get_locale;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub locale: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let locale = get_locale().unwrap_or_else(|| {
            warn!("Couldn't detect current system locale. Selecting default.");
            String::from("en-US")
        });

        Self { locale }
    }
}

impl AppConfig {
    pub fn get_config() -> Result<Self, String> {
        let mut file: File;
        let config: Self;
        let mut data: String = String::new();
        let path = dirs::data_dir();

        if let None = path {
            let err = "Data directorie not found.";
            return Err(err.to_string());
        }

        let mut path = path.unwrap().join(".funnylauncher");

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(&path) {
                let err = format!("Couldn't create directories to config file. Error: {e}");
                return Err(err);
            };
        }

        path = path.join("config.json");

        if !path.exists() {
            file = match File::create(&path) {
                Ok(file) => file,
                Err(e) => {
                    let err = format!("Couldn't create config file. Error: {e}");
                    return Err(err);
                }
            };

            config = Self::default();
            if let Err(e) = file.write(&serde_json::to_string(&config).unwrap().as_bytes()) {
                let err = format!("Couldn't write config to file. Error: {e}");
                return Err(err);
            };

            return Ok(config);
        }

        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                let err = format!("Couldn't open config file. Error: {e}");
                return Err(err);
            }
        };

        if let Err(e) = file.read_to_string(&mut data) {
            let err = format!("Couldn't open config file. Error: {e}");
            return Err(err);
        }

        config = serde_json::from_str(&data).unwrap();

        Ok(config)
    }
}
