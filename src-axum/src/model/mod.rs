pub mod user;
mod error;
use std::env;
use dotenv::dotenv;

pub use self::error::{Error, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type DbPool = Pool<Postgres>;

/// create db pool
pub async fn create_db_pool(db_url: &str) -> Result<DbPool> {
	info!("--> database url is {:?}", db_url);
	PgPoolOptions::new()
		.max_connections(5)
		.connect(&db_url)
		.await
		.map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}

#[derive(Clone)]
pub struct ModelManager {
    pub db_pool: DbPool,
}

impl ModelManager {
    /// default database is postgres
    pub async fn new_postgres() -> Result<Self> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("Expected Database URL..!");
        let db_pool = create_db_pool(&db_url).await?;
        Ok(ModelManager { db_pool })
    }

    /// other database
    pub async fn new_db_pool(db_url: &str) -> Result<Self> {
        let db_pool = create_db_pool(db_url).await?;
        Ok(ModelManager { db_pool })
    }

    pub async fn app_dev_pool() -> Result<Self> {
        dotenv().ok();
        let db_url = env::var("APP_DEV_DATABASE_URL")
            .expect("Expected Database URL..!");
        let db_pool = create_db_pool(db_url).await?;
        Ok(ModelManager { db_pool })
    }
}