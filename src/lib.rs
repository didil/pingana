pub mod db;
pub mod env;

use futures::prelude::*;
use futures::stream;
use mysql_async::prelude::*;
use log::{info};

pub async fn run(config: env::Config) -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::build_pool(&config);
    
    info!("fetching targets ...");
    let ping_targets = fetch_ping_targets(&db_pool, config.max_fails).await?;

    let fs = ping_targets
        .iter()
        .map(|ping_target| do_ping(&ping_target.url));

    let buffered = stream::iter(fs).buffered(config.concurrency as usize);

    info!("pinging {} targets ...", ping_targets.len());
    let ping_results: Vec<_> = buffered.collect().await;

    let ok_count = ping_results.iter().filter(|res| res.is_ok()).count();
    let failed_count = ping_results.len() - ok_count;

    info!("updating targets [{} ok |{} failed] ...",ok_count,failed_count);
    for (target, res) in ping_targets.iter().zip(ping_results.into_iter()) {
        update_ping_target(&db_pool, target.id, res).await?;
    }

    db_pool.disconnect().await?;

    Ok(())
}

struct PingTarget {
    id: u32,
    url: String,
}

async fn fetch_ping_targets(
    pool: &mysql_async::Pool,
    max_fails: u8,
) -> Result<Vec<PingTarget>, mysql_async::error::Error> {
    let conn = pool.get_conn().await?;

    let result = conn
        .prep_exec(
            "SELECT id, url from ping_targets where fails < :fails",
            params! {
                "fails" => max_fails,
            },
        )
        .await?;

    let (_, ping_targets) = result
        .map_and_drop(|row| {
            let (id, url) = mysql_async::from_row(row);
            PingTarget { id: id, url: url }
        })
        .await?;

    Ok(ping_targets)
}

async fn do_ping(url: &str) -> Result<u16, reqwest::Error> {
    let resp = reqwest::get(url).await?.error_for_status()?;
    let status_code = resp.status().as_u16();

    Ok(status_code)
}

async fn update_ping_target(
    pool: &mysql_async::Pool,
    id: u32,
    result: Result<u16, reqwest::Error>,
) -> Result<(), mysql_async::error::Error> {
    let conn = pool.get_conn().await?;

    match result{
        Ok(status_code) => {
            conn.drop_exec(
                r" UPDATE ping_targets 
                   SET fails = 0, last_status = :last_status, last_pinged_at = NOW()
                   WHERE id = :id ",
                params! {
                    "id" => id,
                    "last_status" => status_code.to_string(),
                },
            ).await?;
        },
        Err(e) => {
            conn.drop_exec(
                r" UPDATE ping_targets 
                   SET fails = fails + 1, last_status = :last_status, last_pinged_at = NOW()
                   WHERE id = :id ",
                params! {
                    "id" => id,
                    "last_status" => format!("{}",e),
                },
            ).await?;
        },
    }
   

    Ok(())
}
