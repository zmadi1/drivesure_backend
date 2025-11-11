use actix_web::{HttpResponse, Responder};

pub async fn create_mandate() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Payment mandate placeholder"
    }))
}

