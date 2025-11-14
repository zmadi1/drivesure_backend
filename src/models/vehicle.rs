use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::{Queryable, Selectable, Insertable};   // <- NEW

use crate::schema::vehicles;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = vehicles)]
pub struct Vehicle {
    pub id: Uuid,
    pub owner_user_id: Uuid,
    pub plate_number: String,
    pub make: String,
    pub model: String,
    pub year: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = vehicles)]
pub struct NewVehicle {
    pub owner_user_id: Uuid,
    pub plate_number: String,
    pub make: String,
    pub model: String,
    pub year: i16,
}