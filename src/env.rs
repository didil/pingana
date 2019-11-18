use dotenv::dotenv;
use std::env;
use anyhow::Result;

#[derive(Debug)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_table: String,
    pub max_fails: u8,
    pub concurrency: usize,
    pub fail_callback: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        dotenv()?;
       
        let db_host = env::var("DB_HOST")?;
        let db_port = env::var("DB_PORT")?.parse()?;
        let db_name = env::var("DB_NAME")?;
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_table = env::var("DB_TABLE")?;
        let max_fails = env::var("MAX_FAILS")?.parse()?;
        let concurrency = env::var("CONCURRENCY")?.parse()?;
        let fail_callback = env::var("FAIL_CALLBACK")?;

        let config = Config{
            db_host,
            db_port,
            db_name,
            db_user,
            db_password,
            db_table,
            max_fails,
            concurrency,
            fail_callback,
        };

        Ok(config)
    }
}
