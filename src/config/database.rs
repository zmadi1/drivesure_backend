use std::env;

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        
        let url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        DatabaseConfig { url }
    }
}
