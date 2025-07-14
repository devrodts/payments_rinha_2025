use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    /// Log level configuration for the application.
    /// Currently not used but kept for future logging implementation.
    /// TODO: Implement logging system using this field (T2.x task)
    #[allow(dead_code)]
    pub log_level: String
}

impl Config {
    pub fn new() -> Self {
        let server_addr = SocketAddr::from(([0, 0, 0, 0], 9999));
        
        Config {
            server_addr,
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        }
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server_addr
    }

    /// Getter for log level configuration.
    /// Currently not used but kept for future logging implementation.
    /// TODO: Implement logging system using this method (T2.x task)
    #[allow(dead_code)]
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
} 