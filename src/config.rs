use std::fs;

use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: String,
    pub password: String,
    pub channel: String,
}

pub fn load_config() -> Result<Config> {
    match fs::read_to_string("config.toml") {
        Ok(r) => Ok(toml::from_str::<Config>(&r)?),
        Err(e) => bail!("Unable to read config: {}", e),
    }
}
