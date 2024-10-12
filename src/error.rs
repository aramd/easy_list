#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("WebDriver error: {0}")]
    Telegram(#[from] teloxide::RequestError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Toml parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}
