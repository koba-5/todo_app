pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

impl Config {
    pub fn new(server_addr: String, database_url: String) -> Self {
        Self {
            server_addr,
            database_url,
        }
    }
}
