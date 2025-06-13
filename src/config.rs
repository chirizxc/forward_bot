#![allow(clippy::module_name_repetitions)]

use serde::Deserialize;
use std::{
    env::{self, VarError},
    fs, io,
    path::Path,
};
use thiserror::Error;

#[derive(Deserialize, Clone)]
pub struct BotConfig {
    pub token: Box<str>,
}

#[derive(Deserialize, Clone)]
pub struct ChatConfig {
    pub from_id: i64,
    pub to_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct LoggingConfig {
    pub dirs: Box<str>,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
    pub chat: ChatConfig,
    pub logging: LoggingConfig,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

/// # Panics
/// If the `CONFIG_PATH` environment variable is not a valid UTF-8 string
#[must_use]
pub fn get_path() -> Box<str> {
    let path = match env::var("CONFIG_PATH") {
        Ok(path) => path,
        Err(VarError::NotPresent) => "config.toml".to_owned(),
        Err(VarError::NotUnicode(_)) => {
            panic!("`CONFIG_PATH` env variable is not a valid UTF-8 string!");
        }
    };

    path.into_boxed_str()
}

#[allow(clippy::missing_errors_doc)]
pub fn parse_from_fs(path: impl AsRef<Path>) -> Result<Config, ParseError> {
    let raw = fs::read_to_string(path)?;

    toml::from_str(&raw).map_err(Into::into)
}
