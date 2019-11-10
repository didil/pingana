use pingana::settings::Settings;
use pingana::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new()?;

    run(settings).await?;
    Ok(())
}

