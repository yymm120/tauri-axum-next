use dotenv::dotenv;
// use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::{env, fs};
use std::path::PathBuf;
// use std::time::Duration;

use crate::model::ModelManager;

type Db = Pool<Postgres>;

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    // use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_init_dev_db() -> Result<()> {
        
        Ok(())
    }
}