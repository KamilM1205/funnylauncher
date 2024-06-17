use std::{process::exit, time::Duration};

use reqwest::blocking::Client;
use serde_derive::Deserialize;

use crate::utils::{
    constants::{GET_NEWS_LIST, URL},
    relaunch::relaunch,
};

use super::auth::Auth;

#[derive(Deserialize, Clone)]
pub struct News {
    pub id: String,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    pub title: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(skip)]
    pub not_clickable: bool,
}

impl News {
    pub fn error_news() -> Self {
        Self {
            id: String::new(),
            creator_id: String::new(),
            title: "Error: couldn't load news from service".to_string(),
            body: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
            not_clickable: true,
        }
    }

    pub fn load() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_millis(1500))
            .build()?;

        let resp = client
            .get(format!(
                "{}{}?page={}&size={}&sort=createdAt,desc",
                URL, GET_NEWS_LIST, 0, 10
            ))
            .header("Content-Type", "application/json")
            .send()?;

        if resp.status().is_client_error() {
            Auth::remove_token()?;
            relaunch()?;
            exit(0);
        } else if resp.status().is_server_error() {
            return Ok(vec![News::error_news()]);
        }

        let ret = serde_json::from_str::<Vec<Self>>(&resp.text()?)?;

        Ok(ret)
    }
}
