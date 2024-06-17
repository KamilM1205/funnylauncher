use std::{fmt::format, time::Duration};

use reqwest::blocking::Client;
use serde::Deserialize;

use crate::utils::constants::{GET_USER_URL, UPDATE_USER_ONLINE_URL, URL};

use super::auth::Auth;

#[derive(Deserialize)]
pub struct Account {
    id: String,
    login: String,
    status: String,
    #[serde(rename = "isOnline")]
    is_online: bool,
    role: String,

    #[serde(skip_deserializing)]
    client: Client,
    #[serde(skip_deserializing)]
    token: String,
}

impl Account {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token = Auth::load()?.token;
        let client = Client::builder()
            .timeout(Duration::from_millis(1500))
            .build()?;
        let resp = client
            .get(format!("{}{}", URL, GET_USER_URL))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .send()?;

        if !resp.status().is_success() {
            return Err(format!("Not authorized. {}", resp.status().to_string()).into());
        }

        let mut ret = serde_json::from_str::<Self>(&resp.text()?)?;
        ret.client = client;
        ret.token = token;

        Ok(ret)
    }

    fn update_online(&mut self, online: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.is_online = online;
        self.client
            .patch(format!("{}{}", URL, UPDATE_USER_ONLINE_URL))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(format!(
                r#"{{
                "status": {}
            }}"#,
                self.status
            ))
            .send()?;

        Ok(())
    }

    pub fn send_online(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_online(true)
    }

    pub fn send_offline(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.update_online(false)
    }
}
