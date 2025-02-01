use std::sync::{Mutex, Arc};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use thiserror::Error;

use crate::db::DatabaseError;

#[derive(Error, Debug)]
pub enum AppStateError {
    #[error("Cannot get database pools.")]
    DatabasePoolError(#[from] DatabaseError)
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Arc<Mutex<Pool<ConnectionManager<PgConnection>>>>
}

impl AppState {

    #[tracing::instrument(name = "appstate", fields(fn_type="new"))]
    pub fn new() -> Result<Self, AppStateError> {
        let pools = crate::db::create_pool().map_err(|e| AppStateError::DatabasePoolError(e))?;
        Ok(Self { 
            pool: Arc::new(Mutex::new(pools))
        })
    }

}
