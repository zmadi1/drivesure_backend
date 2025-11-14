use actix_web::web;
use crate::handlers::{user, vehicle};   // <- NEW

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/users", web::post().to(user::create_user))
            .route("/users", web::get().to(user::get_users))
            .route("/users/{user_id}/vehicles", web::post().to(vehicle::register))
            .route("/users/{user_id}/vehicles", web::get().to(vehicle::list_by_user)),
    );
}

pub fn health_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "drivesure_backend",
        "version": "0.1.0"
    }))
}