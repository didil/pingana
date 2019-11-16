use pingana::env::Config;
use pingana::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()?;

    run(config).await?;
    Ok(())
}