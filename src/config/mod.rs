pub mod database;

use std::env;

#[derive(Debug)]
pub struct Config {
    pub database: database::DatabaseConfig,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super-secret-key-change-in-production".to_string());

        Ok(Config {
            database: database::DatabaseConfig::new(),
            port,
            jwt_secret,
        })
    }
}
