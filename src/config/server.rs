#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    address: String,
    port: String,
}


impl ServerConfig {
    // Get server address
    #[inline]
    pub fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            address: String::from("127.0.0.1"),
            port: String::from("8080"),
        }
    }
}
