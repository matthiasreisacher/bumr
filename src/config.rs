use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub aws: Aws,
    pub cache: Cache,
}

#[derive(Deserialize, Debug)]
pub struct Aws {
    pub region: String,
    pub dynamodb: DynamoDb,
}

#[derive(Deserialize, Debug)]
pub struct DynamoDb {
    pub endpoint_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Cache {
    pub sqlite: Sqlite,
}

#[derive(Deserialize, Debug)]
pub struct Sqlite {
    pub path: String,
}

/// Reads the config file <b>config.toml</b> and returns its values or defaults.
pub fn read_config() -> Result<Config> {
    let file_content =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"))
            .context("Missing application's config file")?;

    let config = toml::from_str(&file_content)
        .context("Could not read the application's config file")?;
    Ok(config)
}