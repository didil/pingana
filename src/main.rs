use pingana::env::Config;
use pingana::run;
use log::{info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()?;
    env_logger::init();

    info!("starting batch ...");
    run(config).await?;
    info!("batch done");
    Ok(())
}