
// src/database/mod.rs
pub mod migrations;

use diesel::pg::PgConnection;
use r2d2::{Pool, PooledConnection};
use diesel::r2d2::ConnectionManager;
use std::env;
use dotenvy::dotenv;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Run migrations on startup
    let mut conn = pool.get().expect("Failed to get connection for migrations");
    
    if let Err(e) = migrations::run_migrations(&mut conn) {
        eprintln!("Failed to run migrations: {}", e);
        // Don't panic - maybe migrations are already run
        println!("Continuing without migrations...");
    }
    
    pool
}
