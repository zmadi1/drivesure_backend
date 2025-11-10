use actix_web::{web, HttpResponse};

pub async fn create_vehicle() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Vehicle endpoint - not implemented"}))
}

pub async fn get_vehicles() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Get vehicles - not implemented"}))
}

pub async fn get_available_vehicles() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Available vehicles - not implemented"}))
}

pub async fn get_vehicle_by_id() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"message": "Get vehicle by ID - not implemented"}))
}
