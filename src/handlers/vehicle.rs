use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::vehicle::NewVehicle;

use crate::utils::responses::ApiResponse;

use crate::state::{AppState, vehicle_store::VehicleStore};

#[derive(serde::Deserialize)]
pub struct RegisterVehiclePayload {
    pub plate_number: String,
    pub make: String,
    pub model: String,
    pub year: i16,
}

pub async fn register(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,                 // {user_id}
    payload: web::Json<RegisterVehiclePayload>,
) -> actix_web::Result<HttpResponse> {
    let user_id = path.into_inner();
    let new_vehicle = NewVehicle {
        owner_user_id: user_id,
        plate_number: payload.plate_number.clone(),
        make: payload.make.clone(),
        model: payload.model.clone(),
        year: payload.year,
    };

    let vehicle = VehicleStore::register(&state.db_pool, new_vehicle).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(vehicle)))
}

pub async fn list_by_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let vehicles = VehicleStore::list_by_user(&state.db_pool, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(vehicles)))
}