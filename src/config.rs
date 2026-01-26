use std::{env, net::SocketAddr, str::FromStr};
use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    pub ws: Url,
    pub http: Url,
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let ws = env::var("SOLANA_WS_URL").map(|ws| Url::from_str(&ws))??;
        let http = env::var("SOLANA_HTTP_URL").map(|http| Url::from_str(&http))??;
        let addr = env::var("SOCKET_ADDR").map(|addr| SocketAddr::from_str(&addr))??;

        Ok(Config { ws, http, addr })
    }
}
