use crate::env::Config;
use anyhow::Result;
use mysql_async::{prelude::*, Conn, OptsBuilder, Pool};

pub struct DataService {
    pool: Option<Pool>,
    pub db_table: String,
}

impl DataService {
    pub fn new(config: &Config) -> DataService {
        let mut builder = OptsBuilder::new();
        builder
            .ip_or_hostname(&config.db_host)
            .tcp_port(config.db_port)
            .db_name(Some(&config.db_name))
            .user(Some(&config.db_user))
            .pass(Some(&config.db_password));
        let pool = Pool::new(builder);

        DataService {
            pool: Some(pool),
            db_table: config.db_table.clone(),
        }
    }

    async fn get_conn(&self) -> Result<Conn> {
        let pool = self.pool.as_ref().expect("no db pool");
        let conn = pool.get_conn().await?;
        Ok(conn)
    }

    pub async fn fetch_ping_targets(&self, max_fails: u8) -> Result<Vec<PingTarget>> {
        let conn = self.get_conn().await?;
        let result = conn
            .prep_exec(
                format!(
                    r"
                SELECT id, cron_url 
                FROM {} 
                WHERE 
                cron_fails_count < :cron_fails_count 
                AND 
                (
                  cron_last_pinged_at IS NULL 
                  OR 
                  cron_last_pinged_at < DATE_ADD(NOW(), INTERVAL - 30 MINUTE)
                )
                AND is_activated = 1 AND is_trial = 0
                AND expires_at > NOW()
                 ",
                    self.db_table
                ),
                params! {
                    "cron_fails_count" => max_fails,
                },
            )
            .await?;
        let (_, ping_targets) = result
            .map_and_drop(|row| {
                let (id, cron_url) = mysql_async::from_row(row);
                PingTarget { id, cron_url }
            })
            .await?;
        Ok(ping_targets)
    }

    pub async fn update_ping_target(&self, id: u32, result: Result<u16>) -> Result<()> {
        let conn = self.get_conn().await?;

        match result {
            Ok(status_code) => {
                conn.drop_exec(
                    format!(
                        r" UPDATE {} 
                       SET 
                       is_auto = 1,                    
                       cron_fails_count = 0,
                       cron_last_ping_status = :cron_last_ping_status,
                       cron_last_pinged_at = NOW()
                       WHERE id = :id ",
                        self.db_table
                    ),
                    params! {
                        "id" => id,
                        "cron_last_ping_status" => status_code.to_string(),
                    },
                )
                .await?;
            }
            Err(e) => {
                conn.drop_exec(
                    format!(
                        r" UPDATE {} 
                       SET 
                       is_auto = 0,
                       cron_fails_count = cron_fails_count + 1,
                       cron_last_ping_status = :cron_last_ping_status,
                       cron_last_pinged_at = NOW()
                       WHERE id = :id ",
                        self.db_table
                    ),
                    params! {
                        "id" => id,
                        "cron_last_ping_status" => format!("{}",e),
                    },
                )
                .await?;
            }
        }
        Ok(())
    }

    pub async fn close_db(&mut self) -> Result<()> {
        self.pool.take().expect("no db pool").disconnect().await?;
        Ok(())
    }
}

pub struct PingTarget {
    pub id: u32,
    pub cron_url: String,
}
