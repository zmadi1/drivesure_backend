use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // User routes
            .route("/users", web::post().to(handlers::create_user))
            .route("/users", web::get().to(handlers::get_users))
            // Vehicle routes
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
