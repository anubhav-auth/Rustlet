use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Self {
        let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let server_addr: SocketAddr = addr.parse().expect("Invalid server address format");
        Config { server_addr }
    }
}
