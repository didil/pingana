use pingana::env::Config;
use pingana::run;
use log::{info};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    env_logger::init();

    info!("starting batch ...");
    run(config).await?;
    info!("batch done");
    Ok(())
}