use dotenv::dotenv;
// use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::{env, fs};
use std::path::PathBuf;
// use std::time::Duration;

use crate::model::ModelManager;

type Db = Pool<Postgres>;

const SQL_RECREATE_DB: &str = "demo06-auth/sql/dev_initial/00-recreate-db.sql";
const SQL_DIR: &str = "demo06-auth/sql/dev_initial";

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {

    // -- Create the app_db/app_user with the postgres user.
    {
        let ModelManager { db_pool } = ModelManager::new_postgres().await?;
        let root_db = db_pool;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    // -- Get sql files.
    {
        let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        paths.sort();
        // -- SQL Execute each file.
		dotenv().ok();
		let ModelManager { db_pool } = ModelManager::new_db_pool(&env::var("DATABASE_URL")
			.expect("Expected Database URL..!"))
			.await?;
		let app_db = db_pool;
        // let app_db = new_db_pool(PG_DEV_APP_URL).await?;
        for path in paths {
            if let Some(path) = path.to_str() {
                let path = path.replace('\\', "/"); // for windows.
                                                    // Only take the .sql and skip the SQL_RECREATE_DB
                if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                    pexec(&app_db, &path).await?;
                }
            }
        }
    }
    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // -- Read the file.
    let content = fs::read_to_string(file)?;

    // FIXME: Make the split more sql proof.
    let sqls: Vec<&str> = content.split(';').collect();

    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

// async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
// 	PgPoolOptions::new()
// 		.max_connections(1)
// 		.acquire_timeout(Duration::from_millis(6000))
// 		.connect(db_con_url)
// 		.await
// }
