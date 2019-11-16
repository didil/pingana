use crate::env::Config;
use mysql_async::{OptsBuilder, Pool};

pub fn build_pool(config: &Config) -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(&config.db_host)
        .tcp_port(config.db_port)
        .db_name(Some(&config.db_name))
        .user(Some(&config.db_user))
        .pass(Some(&config.db_password));

    Pool::new(builder)
}
