use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub static_dir: String,
    pub environment: String,
    #[allow(dead_code)]
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            static_dir: env::var("STATIC_DIR")
                .unwrap_or_else(|_| "./public".to_string()),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "debug".to_string()),
        }
    }
}
