use log::warn;
use serde_json::Value;

pub const L_EN: &str = include_str!("../../locales/en-US.json");
pub const L_RU: &str = include_str!("../../locales/ru-RU.json");

pub struct Locale;

impl Locale {
    pub fn load(locale: impl Into<String>) -> Value {
        let locale = locale.into();
        let data = match locale.as_str() {
            "en-US" => L_EN,
            "ru-RU" => L_RU,
            _ => {
                warn!("Language \"{locale}\" not found. Loading default English");
                L_EN
            }
        };

        serde_json::from_str(data).unwrap()
    }

    pub fn get_list() -> Vec<String> {
        vec!["ru-RU".to_string(), "en-US".to_string()]
    }
}
