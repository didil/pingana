pub mod data;
pub mod env;

use anyhow::{anyhow, Result};
use data::{DataService, PingTarget};
use futures::prelude::*;
use futures::stream;
use log::info;

pub async fn run(config: env::Config) -> Result<()> {
    let mut data_svc = DataService::new(&config);
    info!("fetching targets ...");
    let ping_targets = data_svc.fetch_ping_targets(config.max_fails).await?;

    info!("pinging {} targets ...", ping_targets.len());
    let ping_results = do_pings(&ping_targets, config.concurrency).await;

    let ok_count = ping_results.iter().filter(|res| res.is_ok()).count();
    let failed_count = ping_results.len() - ok_count;

    info!(
        "updating targets [{} ok |{} failed] ...",
        ok_count, failed_count
    );
    for (target, res) in ping_targets.iter().zip(ping_results.into_iter()) {
        data_svc.update_ping_target(target.id, res).await?;
    }

    data_svc.close_db().await?;

    Ok(())
}

async fn do_pings(ping_targets: &[PingTarget], concurrency: usize) -> Vec<Result<u16>> {
    let fs = ping_targets
        .iter()
        .map(|ping_target| do_ping(&ping_target.cron_url));

    let buffered = stream::iter(fs).buffered(concurrency);
    let ping_results: Vec<_> = buffered.collect().await;
    ping_results
}

async fn do_ping(url: &str) -> Result<u16> {
    let resp = reqwest::get(url).await?;
    let status_code = resp.status().as_u16();

    if status_code >= 400 {
        return Err(anyhow!("{}", status_code));
    }

    Ok(status_code)
}
