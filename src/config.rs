use std::{fs, io};

use anyhow::Error;
use serde::Deserialize;
use toml::de;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ffmpeg: String,
    pub src: Vec<Src>,
    pub storage: Storage,
}

#[derive(Debug, Deserialize)]
pub struct Src {
    pub id: String,
    pub name: String,
    pub url: String,
    pub frequency: u64,
}

#[derive(Debug, Deserialize)]
pub struct Storage {
    pub local: String,
}

/// 实现自定义错误类型
#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    TomlParseError(de::Error),
}

/// 实现anyhow::Error方便上一级直接使用
impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::IOError(e) => Error::msg(e.to_string()),
            ConfigError::TomlParseError(e) => Error::msg(e.to_string()),
        }
    }
}

impl ConfigError {
    fn from_io(err: io::Error) -> ConfigError {
        ConfigError::IOError(err)
    }

    fn from_toml(err: de::Error) -> ConfigError {
        ConfigError::TomlParseError(err)
    }
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config_str =
            fs::read_to_string("config.toml").map_err(|err| ConfigError::from_io(err))?;
        Ok(toml::from_str(&config_str).map_err(|err| ConfigError::from_toml(err))?)
    }
}
