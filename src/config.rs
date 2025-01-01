pub struct Config {
    host: String,
    port: usize,
    worker_count: usize,

    environment: String,

    db_host: String,
    db_port: usize,
    db_user: String,
    db_pass: String,
    db_base: String,
    db_echo: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            worker_count: 2,
            environment: "dev".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_user: "xythrion".to_string(),
            db_pass: "xythrion".to_string(),
            db_base: "xythrion".to_string(),
            db_echo: false,
        }
    }
}

impl Config {
    fn db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.db_host, self.db_port, self.db_user, self.db_pass, self.db_base
        )
    }
}
