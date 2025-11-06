use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub message: String,
}

pub fn success_response<T: Serialize>(data: T, message: &str) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data,
        message: message.to_string(),
    })
}

pub fn error_response(message: &str, error: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse {
        success: false,
        error: error.to_string(),
        message: message.to_string(),
    })
}

pub fn internal_error_response() -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse {
        success: false,
        error: "Internal server error".to_string(),
        message: "Something went wrong".to_string(),
    })
}
