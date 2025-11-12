use actix_web::{web, HttpResponse}; use diesel::prelude::*; use uuid::Uuid;
use crate::database::DbPool; use crate::models::vehicle::{Vehicle, NewVehicle, CreateVehicleRequest, VehicleResponse};
use crate::schema::vehicles; use crate::utils::{success_response, error_response};

pub async fn create_vehicle(req: web::Json<CreateVehicleRequest>, owner_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool");
    let new_v = NewVehicle { owner_id: owner_id.into_inner(), make: req.make.clone(), model: req.model.clone(), year: req.year, reg_number: req.reg_number.clone(), imei: req.imei.clone(), };
    let v: Vehicle = diesel::insert_into(vehicles::table).values(&new_v).get_result(conn).expect("insert failed");
    success_response(VehicleResponse::from(v), "Vehicle added")
}
pub async fn get_vehicles(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool"); let list: Vec<Vehicle> = vehicles::table.load(conn).expect("load failed");
    let resp: Vec<VehicleResponse> = list.into_iter().map(VehicleResponse::from).collect(); success_response(resp, "Vehicles retrieved")
}
pub async fn get_available_vehicles(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool"); let list: Vec<Vehicle> = vehicles::table.filter(vehicles::is_available.eq(true)).load(conn).expect("load failed");
    let resp: Vec<VehicleResponse> = list.into_iter().map(VehicleResponse::from).collect(); success_response(resp, "Available vehicles")
}
pub async fn get_vehicle_by_id(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool");
    match vehicles::table.find(id.into_inner()).first::<Vehicle>(conn).optional() {
        Ok(Some(v)) => success_response(VehicleResponse::from(v), "Vehicle found"),
        Ok(None) => error_response("Vehicle not found", "NOT_FOUND"),
        Err(_) => error_response("DB error", "INTERNAL_ERROR"),
    }
}
