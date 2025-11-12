use diesel::prelude::*; use serde::{Deserialize, Serialize}; use uuid::Uuid; use chrono::{DateTime, Utc, NaiveDate};
use crate::schema::payment_mandates;
#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)] #[diesel(table_name = payment_mandates)]
pub struct PaymentMandate { pub id: Uuid, pub vehicle_id: Uuid, pub owner_id: Uuid, pub driver_id: Uuid, pub amount_cents: i32, pub retry_count: i16, pub status: MandateStatus, pub due_date: NaiveDate, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc>, }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, diesel::AsExpression, diesel::FromSqlRow)] #[diesel(sql_type = crate::schema::mandate_status::name)]
pub enum MandateStatus { Active, Failed, Completed }
#[derive(Insertable)] #[diesel(table_name = payment_mandates)]
pub struct NewPaymentMandate { pub vehicle_id: Uuid, pub owner_id: Uuid, pub driver_id: Uuid, pub amount_cents: i32, pub due_date: NaiveDate, }
#[derive(Deserialize)] pub struct CreateMandateRequest { pub vehicle_id: Uuid, pub driver_id: Uuid, pub amount_cents: i32, pub due_date: NaiveDate, }
#[derive(Serialize)] pub struct MandateResponse { pub mandate_id: Uuid, pub status: MandateStatus, }
