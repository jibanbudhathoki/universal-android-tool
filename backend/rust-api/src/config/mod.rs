use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("UAT_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("UAT_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Self { host, port }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 8080)))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
