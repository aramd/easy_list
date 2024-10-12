use crate::error::Error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Secrets {
    pub tg_token: String,
    pub redis_uri: String,
}

impl Secrets {
    pub fn new() -> Result<Self, Error> {
        let file_content = fs::read_to_string("./Secrets.toml")?;
        let secrets: Secrets = toml::from_str(&file_content)?;
        Ok(secrets)
    }
}
