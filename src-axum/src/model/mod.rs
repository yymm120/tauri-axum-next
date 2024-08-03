pub mod user;
mod error;

use crate::configuration::DatabaseSettings;

pub use self::error::{Error, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;



#[derive(Clone)]
pub struct ModelManager {
    pub db_pool: PgPool,
}

impl ModelManager {

    pub async fn lazy_connect(configuration: &DatabaseSettings) -> Result<Self> {
        Ok(ModelManager { db_pool: PgPoolOptions::new().max_connections(5).connect_lazy_with(configuration.with_db())})
    }

}