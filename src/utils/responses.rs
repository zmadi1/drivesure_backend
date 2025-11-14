use actix_web::HttpResponse;
use serde::Serialize;
use thiserror::Error;               // NEW

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: String,
}

// NEW: small ctor so handler code compiles
impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data,
            message: "OK".to_string(),
        }
    }
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

// ------------------------------------------------------------------
// NEW: ApiError type required by the store layer
// ------------------------------------------------------------------
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("{0}")]
    Internal(String),
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        internal_error_response()
    }
}