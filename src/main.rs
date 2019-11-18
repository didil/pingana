use pingana::env::Config;
use pingana::run;
use log::{info};
use anyhow::Result;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    env_logger::init();

    info!("starting pingana ({}) ...",VERSION);
    run(config).await?;
    info!("done");
    Ok(())
}