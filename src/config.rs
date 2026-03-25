use std::net::SocketAddr;

pub struct Config {
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            server_addr: std::env::var("SERVER_ADDR")
                .expect("SERVER_ADDR not set")
                .parse()
                .expect("SERVER_ADDR must be a valid address"),
        }
    }
}
