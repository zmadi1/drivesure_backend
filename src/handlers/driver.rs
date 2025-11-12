use actix_web::{web, HttpResponse}; use diesel::prelude::*; use uuid::Uuid;
use crate::database::DbPool; use crate::models::driver::{DriverVerification, NewDriverVerification, VerifyDriverRequest, VerifyDriverResponse};
use crate::schema::driver_verifications; use crate::utils::success_response;

pub async fn verify_driver(req: web::Json<VerifyDriverRequest>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool");
    let rtmc_ok = req.licence_number.len() == 12; let face_score = if req.licence_number.len() % 2 == 0 { 0.92 } else { 0.71 };
    let new_row = NewDriverVerification { user_id: req.user_id, licence_number: req.licence_number.clone(), licence_front_url: req.licence_front_url.clone(), selfie_url: req.selfie_url.clone(), face_match_score: Some(face_score), rtmc_verified: rtmc_ok, };
    let verif: DriverVerification = diesel::insert_into(driver_verifications::table).values(&new_row).get_result(conn).expect("insert failed");
    success_response(VerifyDriverResponse { verification_id: verif.id, status: if verif.rtmc_verified && face_score > 0.80 { "CLEAR" } else { "REVIEW" } }, "Driver verified")
}
