use crate::models::vehicle::{NewVehicle, Vehicle};
use crate::state::DbPool;
use crate::utils::responses::ApiError;
use diesel::prelude::*;
use uuid::Uuid;

pub struct VehicleStore;

impl VehicleStore {
    /// Insert a new vehicle and return the created row.
    pub async fn register(
        pool: &DbPool,
        new: NewVehicle,
    ) -> Result<Vehicle, ApiError> {
        let pool = pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ApiError::Pool)?;
            diesel::insert_into(crate::schema::vehicles::table)
                .values(&new)
                .get_result::<Vehicle>(&mut conn)
                .map_err(ApiError::Database)
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    }

    /// List every vehicle that belongs to a given user.
    pub async fn list_by_user(
        pool: &DbPool,
        user_id: Uuid,
    ) -> Result<Vec<Vehicle>, ApiError> {
        let pool = pool.clone();
        tokio::task::spawn_blocking(move || {
            use crate::schema::vehicles::dsl;
            let mut conn = pool.get().map_err(ApiError::Pool)?;
            dsl::vehicles
                .filter(dsl::owner_user_id.eq(user_id))
                .load::<Vehicle>(&mut conn)
                .map_err(ApiError::Database)
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?
    }
}