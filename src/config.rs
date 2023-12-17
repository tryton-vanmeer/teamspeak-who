use anyhow::{bail, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: String,
    pub password: String,
}

fn get_config_file() -> Result<String> {
    Ok(xdg::BaseDirectories::with_prefix("teamspeak-who")?
        .get_config_file("config.toml")
        .to_string_lossy()
        .to_string())
}

pub fn load_config() -> Result<Config> {
    match fs::read_to_string(get_config_file()?) {
        Ok(r) => Ok(toml::from_str::<Config>(&r)?),
        Err(e) => bail!("Unable to read config ({}): {}", get_config_file()?, e),
    }
}
