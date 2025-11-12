use actix_web::{web, HttpResponse}; use diesel::prelude::*; use uuid::Uuid; use chrono::Utc;
use crate::database::DbPool; use crate::models::payment::{PaymentMandate, NewPaymentMandate, CreateMandateRequest, MandateResponse, MandateStatus};
use crate::schema::payment_mandates; use crate::utils::success_response;

pub async fn create_mandate(req: web::Json<CreateMandateRequest>, owner_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = &mut pool.get().expect("db pool");
    let new_row = NewPaymentMandate { vehicle_id: req.vehicle_id, owner_id: owner_id.into_inner(), driver_id: req.driver_id, amount_cents: req.amount_cents, due_date: req.due_date, };
    let mandate: PaymentMandate = diesel::insert_into(payment_mandates::table).values(&new_row).get_result(conn).expect("insert failed");
    success_response(MandateResponse { mandate_id: mandate.id, status: mandate.status }, "Mandate created")
}
