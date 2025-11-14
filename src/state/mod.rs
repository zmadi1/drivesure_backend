use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod user_store;
pub mod vehicle_store;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}