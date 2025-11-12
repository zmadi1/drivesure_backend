use actix_web::web;
use crate::handlers::{user, driver, payment, vehicle};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        // group subroutes
        .service(
            web::scope("/users")
                .route("", web::post().to(user::create_user))
                .route("", web::get().to(user::get_users)),
        )
        .service(
            web::scope("/vehicles")
                .route("", web::post().to(vehicle::create_vehicle))
                .route("", web::get().to(vehicle::get_vehicles))
                .route("/available", web::get().to(vehicle::get_available_vehicles))
                .route("/{id}", web::get().to(vehicle::get_vehicle_by_id)),
        )
        .service(
            web::scope("/drivers")
                .route("/verify", web::post().to(driver::verify_driver)),
        )
        .service(
            web::scope("/payments")
                .route("/mandate", web::post().to(payment::create_mandate)),
        )
        .route("/health", web::get().to(health_check))
    );
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "drivesure_backend",
        "version": "0.1.0"
    }))
}

