use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    host: String,
    port: usize,
    worker_count: usize,

    environment: String,

    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub db_base: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8001,
            worker_count: 2,
            environment: "dev".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_user: "xythrion".to_string(),
            db_pass: "xythrion".to_string(),
            db_base: "xythrion".to_string(),
        }
    }
}

impl Config {
    #[must_use]
    pub fn new(config_path: String) -> Self {
        read_to_string(config_path).map_or_else(
            |_| {
                warn!("Could not find config, opting for default config");
                Self::default()
            },
            |file_content| match toml::from_str(&file_content) {
                Ok(c) => c,
                Err(err) => {
                    warn!("Error when parsing config: {err}");
                    warn!("Opting for default config");

                    Self::default()
                }
            },
        )
    }

    #[must_use]
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[must_use]
    pub fn db_url(&self) -> String {
        format!(
            "postgresql://{}:{}?dbname={}&user={}&password={}",
            self.db_host, self.db_port, self.db_base, self.db_user, self.db_pass,
        )
    }
}
