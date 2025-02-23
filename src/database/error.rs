use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Sqlx(#[from] sqlx::Error),
    Reqwest(#[from] reqwest::Error),
    SerdeJson(#[from] serde_json::Error),
    ParseInt(#[from] std::num::ParseIntError),
    Http(#[from] crate::http::Error),
    #[error("Player not found")]
    PlayerNotFound,
    #[error("Discord account is already linked to {0}")]
    DiscordInUse(String),
    #[error("Ckey is already linked to {0}")]
    CkeyInUse(i64),
    #[error("Account is not linked")]
    NotLinked,
    #[error("Token does not exist or is invalid")]
    TokenInvalid,
    #[error("Cannot find or connect server")]
    GameServerError,
}
