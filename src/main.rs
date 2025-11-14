
/*
mod models;
mod handlers;
mod state;
mod utils;
mod api;

use actix_web::{web, App, HttpServer};
use state::UserStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_store = UserStore::new();

    println!("🌈 DriveSure Backend - PROPER STRUCTURE");
    println!("📍 http://127.0.0.1:8080");
    println!("📁 Clean architecture with proper modules");
    println!("🚀 Ready for scaling!");
    println!("========================");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_store.clone()))
            .configure(api::health_config)
            .configure(api::config)
            .route("/", web::get().to(welcome))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn welcome() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to DriveSure API",
        "version": "0.1.0",
        "structure": "Clean modular architecture",
        "endpoints": {
            "health": "GET /health",
            "create_user": "POST /api/users",
            "list_users": "GET /api/users"
        }
    }))
}
*/

// src/main.rs
use actix_web::{web, App, HttpServer};
use drivesure_backend::{api, database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::establish_connection();

    println!("🌈 DriveSure Backend - WITH EMBEDDED MIGRATIONS!");
    println!("📍 http://127.0.0.1:8080 ");
    println!("🗄️  Database: PostgreSQL with embedded migrations");
    println!("🚀 Migrations run automatically on startup!");
    println!("========================");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::health_config)
            .configure(api::config)
            .route("/", web::get().to(welcome))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn welcome() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to DriveSure API",
        "version": "0.1.0",
        "database": "PostgreSQL with embedded migrations",
        "endpoints": {
            "health": "GET /health",
            "create_user": "POST /api/users",
            "list_users": "GET /api/users"
        }
    }))
}
