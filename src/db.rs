use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, PgConnection};
use thiserror::Error;

use crate::utils::config::AppState;

pub mod url;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("DATABASE_URL env not found")]
    DatabaseEnvVarError,

    #[error("Cannot create pool")]
    PoolConnectionError,
}

pub fn create_pool() -> Result<Pool<ConnectionManager<PgConnection>>, DatabaseError> {
    let url = std::env::var("DATABASE_URL").map_err(|_| DatabaseError::DatabaseEnvVarError)?;
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .max_size(10)
        .build(manager)
        .map_err(|_| DatabaseError::PoolConnectionError)
}

pub fn get_connection(app_state: &AppState) -> Option<DbConnection> {
    let pool_lock = app_state.pool.lock().ok()?;
    pool_lock.get().map_err(|e| {
        tracing::error!("Error while getting connection from pool. E: {:?}", e);
    }).ok()
}

pub fn check_connection() {

}
