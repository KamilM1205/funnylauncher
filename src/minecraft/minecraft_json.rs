use std::error::Error;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Downloads {
    pub artifact: Artifact,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Downloads,
    pub rules: Option<Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Logging {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Arguments {
    pub game: Value,
    pub jvm: Value,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    pub total_size: i32,
    pub url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftJson {
    #[serde(rename = "_comment_")]
    pub comment: Option<Vec<String>>,
    pub id: String,
    pub time: String,
    pub release_time: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub main_class: String,
    pub minimum_launcher_version: Option<i32>,
    pub inherits_from: Option<String>,
    pub logging: Option<Logging>,
    pub arguments: Arguments,
    pub asset_index: Option<AssetIndex>,
    pub assets: Option<String>,
    pub compliance_level: Option<i32>,
    pub libraries: Vec<Library>,
}

impl MinecraftJson {
    pub fn new(src: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        match serde_json::from_str(&src.into()) {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn libs_to_args(&self, path: impl Into<String>) -> String {
        let path = path.into();
        self.libraries
            .iter()
            .map(|l| format!("{}\\{};", path, l.downloads.artifact.path))
            .collect()
    }

    pub fn jvm_args_to_arg(&self) -> Vec<String> {
        self.arguments
            .jvm
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    }

    pub fn game_args_to_arg(&self) -> Vec<String> {
        self.arguments
            .game
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    }
}
