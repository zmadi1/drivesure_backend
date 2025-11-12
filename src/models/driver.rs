use diesel::prelude::*; use serde::{Deserialize, Serialize}; use uuid::Uuid; use chrono::{DateTime, Utc};
use crate::schema::driver_verifications;
#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)] #[diesel(table_name = driver_verifications)]
pub struct DriverVerification { pub id: Uuid, pub user_id: Uuid, pub licence_number: String, pub licence_front_url: String, pub selfie_url: String, pub face_match_score: Option<f32>, pub rtmc_verified: bool, pub created_at: DateTime<Utc>, }
#[derive(Insertable, Deserialize)] #[diesel(table_name = driver_verifications)]
pub struct NewDriverVerification { pub user_id: Uuid, pub licence_number: String, pub licence_front_url: String, pub selfie_url: String, pub face_match_score: Option<f32>, pub rtmc_verified: bool, }
#[derive(Deserialize)] pub struct VerifyDriverRequest { pub user_id: Uuid, pub licence_number: String, pub licence_front_url: String, pub selfie_url: String, }
#[derive(Serialize)] pub struct VerifyDriverResponse { pub verification_id: Uuid, pub status: &'static str, }
