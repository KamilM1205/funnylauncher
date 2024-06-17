use std::{
    fs::{self, File},
    io::{Read, Write},
    time::Duration,
};

use reqwest::blocking::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::constants::{LAUNCHER_DIR, LOGIN_URL, URL};

pub const LP_ERROR: &str = "login_error";
pub const IS_ERROR: &str = "login_is_error";

#[derive(Default, Serialize, Deserialize)]
pub struct Auth {
    pub login: String,
    pub password: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub token: String,
}

impl Auth {
    pub fn send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_millis(1500))
            .build()?;
        let res = client
            .post(format!("{}{}", URL, LOGIN_URL))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(self)?)
            .send()?;
        if res.status().is_success() {
            self.token = serde_json::from_str::<Value>(&res.text()?)?["token"]
                .as_str()
                .ok_or("no field: 'token'")?
                .to_string();
        } else if res.status().is_client_error() {
            return Err(LP_ERROR.into());
        } else {
            return Err(IS_ERROR.into());
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = dirs::data_dir();

        if path.is_none() {
            return Err("Couldn't open data directory".into());
        }

        let path = path.unwrap().join(LAUNCHER_DIR);

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(&path) {
                return Err(format!("Couldn't create path: {e}").into());
            }
        }

        let path = path.join(".auth");

        let file = File::create(path);
        if let Err(e) = file {
            return Err(format!("Couldn't create file. Error: {e}").into());
        }

        let mut file = file.unwrap();
        if let Err(e) = file.write(self.token.as_bytes()) {
            return Err(format!("Couldn't write token to file. Error: {e}").into());
        }

        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = dirs::data_dir();
        let mut token: String = Default::default();

        if path.is_none() {
            return Err("Couldn't open data directory".into());
        }

        let path = path.unwrap().join(LAUNCHER_DIR);

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(&path) {
                return Err(format!("Couldn't create path: {e}").into());
            }
        }

        let path = path.join(".auth");

        if !path.exists() {
            return Err("Not authorizated".into());
        }

        let file = File::open(path);
        if let Err(e) = file {
            return Err(format!("Couldn't open file. Error: {e}").into());
        }

        let mut file = file.unwrap();
        if let Err(e) = file.read_to_string(&mut token) {
            return Err(format!("Couldn't read token from file. Error: {e}").into());
        }

        token.retain(|c| !"\n\t\0\r".contains(c));

        let mut auth = Self::default();
        auth.token = token;

        Ok(auth)
    }

    pub fn remove_token() -> Result<(), Box<dyn std::error::Error>> {
        let path = dirs::data_dir();

        if path.is_none() {
            return Err("Couldn't open data directory".into());
        }

        let path = path.unwrap().join(LAUNCHER_DIR);

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(&path) {
                return Err(format!("Couldn't create path: {e}").into());
            }
        }

        let path = path.join(".auth");

        let file = File::create(path);
        if let Err(e) = file {
            return Err(format!("Couldn't create file. Error: {e}").into());
        }

        let mut file = file.unwrap();
        if let Err(e) = file.write("".as_bytes()) {
            return Err(format!("Couldn't write token to file. Error: {e}").into());
        }

        Ok(())
    }
}
