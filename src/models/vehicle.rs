use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::schema::vehicles;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(table_name = vehicles)]
pub struct Vehicle {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub reg_number: String,
    pub imei: Option<String>,
    pub can_immobilise: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = vehicles)]
pub struct NewVehicle {
    pub owner_id: Uuid,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub reg_number: String,
    pub imei: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVehicleRequest {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub reg_number: String,
    pub imei: Option<String>,
}

#[derive(Serialize)]
pub struct VehicleResponse {
    pub id: Uuid,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub reg_number: String,
    pub imei: Option<String>,
    pub can_immobilise: bool,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Vehicle> for VehicleResponse {
    fn from(v: Vehicle) -> Self {
        VehicleResponse {
            id: v.id,
            make: v.make,
            model: v.model,
            year: v.year,
            reg_number: v.reg_number,
            imei: v.imei,
            can_immobilise: v.can_immobilise,
            is_available: v.is_available,
            created_at: v.created_at,
        }
    }
}
