use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Simple data models
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: String,
    email: String,
    password_hash: String,
    role: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
struct CreateUserRequest {
    email: String,
    password: String,
    role: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    message: String,
}

// In-memory storage
type UserStore = Arc<Mutex<HashMap<String, User>>>;

// API Handlers
async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    store: web::Data<UserStore>,
) -> Result<HttpResponse> {
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                data: serde_json::Value::Null,
                message: "Failed to hash password".to_string(),
            }))
        }
    };

    let user_id = Uuid::new_v4().to_string();
    let user = User {
        id: user_id.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
        role: user_data.role.clone(),
        first_name: user_data.first_name.clone(),
        last_name: user_data.last_name.clone(),
        phone: user_data.phone.clone(),
        created_at: Utc::now(),
    };

    let mut users = store.lock().unwrap();
    
    // Check if email already exists
    if users.values().any(|u| u.email == user_data.email) {
        return Ok(HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            data: serde_json::Value::Null,
            message: "User with this email already exists".to_string(),
        }));
    }

    users.insert(user_id, user.clone());

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: user,
        message: "User created successfully".to_string(),
    }))
}

async fn get_users(store: web::Data<UserStore>) -> Result<HttpResponse> {
    let users = store.lock().unwrap();
    let users_vec: Vec<User> = users.values().cloned().collect();
    
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: users_vec,
        message: "Users retrieved successfully".to_string(),
    }))
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: serde_json::json!({
            "status": "healthy",
            "service": "drivesure_backend",
            "version": "0.1.0"
        }),
        message: "Service is healthy".to_string(),
    }))
}

async fn welcome() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: serde_json::json!({
            "service": "DriveSure API",
            "version": "0.1.0",
            "endpoints": {
                "health": "GET /health",
                "create_user": "POST /api/users",
                "list_users": "GET /api/users"
            }
        }),
        message: "Welcome to DriveSure API".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_store: UserStore = Arc::new(Mutex::new(HashMap::new()));

    println!("🌈 DriveSure Backend - SIMPLE VERSION");
    println!("📍 http://127.0.0.1:8080");
    println!("🗄️  Storage: In-memory (for development)");
    println!("🚀 Ready to build your app!");
    println!("========================");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_store.clone()))
            .route("/", web::get().to(welcome))
            .route("/health", web::get().to(health_check))
            .route("/api/users", web::get().to(get_users))
            .route("/api/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
