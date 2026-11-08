pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub secret: String,
}

impl Config {
    pub fn new(server_addr: String, database_url: String, secret: String) -> Self {
        Self {
            server_addr,
            database_url,
            secret,
        }
    }
}
