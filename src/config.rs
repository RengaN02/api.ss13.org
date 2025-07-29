use rocket::config::LogLevel;
use serde::Deserialize;
use std::{collections::HashSet, fs::read_to_string, net::IpAddr};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub secret: String,
    pub dev_secret: String,
    pub dev_routes: HashSet<String>,
    pub exposed_secret: String,
    pub exposed_routes: HashSet<String>,
    pub discord: Discord,
    pub discord_oauth: DiscordOAuth,
    pub cli_colors: bool,
    pub log_level: LogLevel,
    pub database: Database,
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Discord {
    pub token: String,
    pub guild: i64,
    pub patreon_role: i64,
}

#[derive(Debug, Deserialize)]
pub struct DiscordOAuth {
    pub client_id: i64,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: IpAddr,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub connection_address: String,
    pub error_message: String,
}

impl Config {
    pub fn read_from_file() -> Result<Self, Error> {
        Ok(toml::from_str(&read_to_string("config.toml")?)?)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Io(#[from] std::io::Error),
    Toml(#[from] toml::de::Error),
}
