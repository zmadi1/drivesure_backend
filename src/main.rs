mod config;
mod models;
mod handlers;
mod database;
mod utils;
mod schema;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Load configuration
    let config = config::Config::from_env()
        .expect("Failed to load configuration");

    // Set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new(&config.database.url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    println!("🌈 Starting DriveSure Backend...");
    println!("📍 Server: http://127.0.0.1:{}", config.port);
    println!("🗄️  Database: {}", config.database.url);
    println!("========================");

    HttpServer::new(move || {
        let cors = Cors::permissive(); // In production, configure this properly

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            // Health check
            .route("/health", web::get().to(|| async {
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "status": "healthy",
                    "service": "drivesure_backend",
                    "version": "0.1.0"
                }))
            }))
            // User routes
            .route("/api/users", web::post().to(handlers::create_user))
            .route("/api/users", web::get().to(handlers::get_users))
            // Welcome route
            .route("/", web::get().to(|| async {
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "message": "Welcome to DriveSure API",
                    "version": "0.1.0",
                    "endpoints": {
                        "health": "/health",
                        "create_user": "POST /api/users",
                        "list_users": "GET /api/users"
                    }
                }))
            }))
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
