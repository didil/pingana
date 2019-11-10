use config::{Config, ConfigError, File};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Misc {
    pub max_fail: u8,
    pub concurrency: u16,
    pub fail_callback: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub misc: Misc,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("settings"))?;

        s.try_into()
    }
}
