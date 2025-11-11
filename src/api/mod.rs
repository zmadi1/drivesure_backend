
/*
pub mod routes;
pub use routes::{config, health_config};

pub fn config(cfg: &mut web::ServiceConfig) {
    crate::api::routes::user_routes(cfg);
    crate::api::routes::driver_routes(cfg);
    crate::api::routes::payment_routes(cfg);
    crate::api::routes::vehicle_routes(cfg); // NEW
}*/



use actix_web::web;

mod routes;
pub use routes::*; // exposes everything (config, health, etc.)

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    config(cfg); // main route registration function
}

