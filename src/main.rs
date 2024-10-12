mod error;
mod secrets;
mod tg_bot;

use error::Error;
use secrets::Secrets;
use tg_bot::TgBot;
use tracing::error;

async fn run() -> Result<(), Error> {
    let secrets = Secrets::new()?;
    let tg_bot = TgBot::new(secrets).await?;
    tg_bot.start().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    if let Err(e) = run().await {
        error!("{}", e);
        return Err(e);
    }

    Ok(())
}
